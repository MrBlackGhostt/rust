import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemInstruction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { expect, test } from "bun:test";
import { LiteSVM } from "litesvm";
import * as borsh from "borsh";
import { deserialize } from "borsh";

class Counter {
  no: number;
  constructor(no: number) {
    this.no = no;
  }

  static schema: borsh.Schema = {
    struct: {
      no: "u32",
    },
  };
}

test("Testing the bun test", () => {
  const svm = new LiteSVM();
  const user = new Keypair();
  const doubleContract = new Keypair();
  const doubleProgramId = PublicKey.unique(); // As program do not hsave tje keypair they only have the publickey as the id
  const cpiProgramId = PublicKey.unique();
  const cliContract = new Keypair();
  svm.addProgramFromFile(doubleProgramId, "./double.so");
  svm.addProgramFromFile(cpiProgramId, "./cli.so");
  svm.airdrop(user.publicKey, BigInt(2 * LAMPORTS_PER_SOL));

  const inx = SystemProgram.createAccount({
    fromPubkey: user.publicKey,
    newAccountPubkey: doubleContract.publicKey,
    lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
    space: 4,
    programId: doubleProgramId,
  });

  const trx = new Transaction().add(inx);
  trx.feePayer = user.publicKey;
  trx.recentBlockhash = svm.latestBlockhash();
  trx.sign(user, doubleContract);

  svm.sendTransaction(trx);
  svm.expireBlockhash();

  //NOW give the instruction to the cli cliContract

  const inx_cli = new TransactionInstruction({
    programId: cpiProgramId,
    keys: [
      { pubkey: doubleContract.publicKey, isSigner: true, isWritable: true },
      { pubkey: doubleProgramId, isSigner: false, isWritable: true },
    ],
  });

  const trx2 = new Transaction().add(inx_cli);
  trx2.feePayer = user.publicKey;
  trx2.recentBlockhash = svm.latestBlockhash();
  trx2.sign(doubleContract, user);

  const tra = svm.sendTransaction(trx2);

  console.log("TRX2", tra.toString());
  svm.expireBlockhash();

  const updateAccountData = svm.getAccount(doubleContract.publicKey);
  if (!updateAccountData) {
    throw new Error("Account not found");
  }
  const data = deserialize(Counter.schema, updateAccountData?.data);
  if (!data) {
    throw new Error("data not found");
  }
  //@ts-ignore
  console.log("Data as", data.no);

  function doubleCounter() {
    const instruction = new TransactionInstruction({
      programId: cpiProgramId,
      keys: [
        { pubkey: doubleContract.publicKey, isSigner: false, isWritable: true },
        { pubkey: doubleProgramId, isSigner: false, isWritable: false },
      ],
      data: Buffer.from([]),
    });

    let transaction = new Transaction().add(instruction);
    transaction.recentBlockhash = svm.latestBlockhash();

    transaction.feePayer = user.publicKey;
    transaction.sign(doubleContract, user);
    svm.sendTransaction(transaction);
    svm.expireBlockhash();
  }

  doubleCounter();
  doubleCounter();
  doubleCounter();
  doubleCounter();

  const updatedAccountData = svm.getAccount(doubleContract.publicKey);
  if (!updatedAccountData) {
    throw new Error("Account not found");
  }
  const updatedCounter = deserialize(Counter.schema, updatedAccountData.data);
  if (!updatedCounter) {
    throw new Error("Counter not found");
  }
  //@ts-ignore
  expect(updatedCounter.no).toBe(16);
});
