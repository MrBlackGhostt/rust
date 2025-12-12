import { expect, test } from "bun:test";
import { LiteSVM } from "litesvm";
import * as borsh from "borsh";
import { deserialize } from "borsh";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
} from "@solana/web3.js";

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

test("one program", () => {
  const svm = new LiteSVM();
  const payer = new Keypair(); // Sol transfer from it
  const dataAccount = new Keypair(); // The Sol to tranfer
  const contractPubkey = PublicKey.unique();
  svm.addProgramFromFile(contractPubkey, "./cli.so");

  svm.airdrop(payer.publicKey, BigInt(2 * LAMPORTS_PER_SOL));

  const recent_blockhash = svm.latestBlockhash();

  const transfer_lampport = 2 * LAMPORTS_PER_SOL;
  // THIS IS CREATING THE dataAccount
  const ixs = [
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
      space: 4,
      programId: contractPubkey,
    }),
  ];
  const transaction = new Transaction({
    recentBlockhash: recent_blockhash,
  }).add(...ixs);

  transaction.sign(payer, dataAccount);

  const compileMessage = transaction.compileMessage();
  svm.sendTransaction(transaction);
  // The dataAccount is createAccount
  const balanceAfter = svm.getBalance(dataAccount.publicKey);

  console.log(JSON.stringify(compileMessage));

  expect(balanceAfter).toBe(svm.minimumBalanceForRentExemption(BigInt(4)));

  function callContract() {
    //  here we seding the instructions to the contract
    const ins = new TransactionInstruction({
      keys: [
        { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
      ],
      programId: contractPubkey,
      data: Buffer.from([]),
    });

    const tx2 = new Transaction().add(ins);

    tx2.recentBlockhash = svm.latestBlockhash();
    tx2.feePayer = payer.publicKey;
    tx2.sign(dataAccount, payer);
    svm.sendTransaction(tx2);
    svm.expireBlockhash();
  }

  callContract();
  callContract();
  callContract();
  callContract();
  callContract();

  let balanceAfter2 = svm.getAccount(dataAccount.publicKey);
  if (!balanceAfter2) {
    throw new Error("Account not found");
  }
  let data = deserialize(Counter.schema, balanceAfter2.data);
  console.log("data", data);
  if (!data) {
    throw new Error("Counter not found");
  }
  //@ts-ignore
  expect(data.no).toBe(16);
});
