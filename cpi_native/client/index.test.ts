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

class CountStruct {
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
test("Double the no", () => {
  const svm = new LiteSVM();

  const signer = new Keypair();

  const programId = PublicKey.unique();

  const dataAccount = new Keypair();

  svm.airdrop(signer.publicKey, BigInt(LAMPORTS_PER_SOL * 2));

  svm.addProgramFromFile(programId, "./double.so");

  let ixn = [
    SystemProgram.createAccount({
      fromPubkey: signer.publicKey,
      /** Public key of the created account */
      newAccountPubkey: dataAccount.publicKey,
      /** Amount of lamports to transfer to the created account */
      lamports: LAMPORTS_PER_SOL * 1.02,
      /** Amount of space in bytes to allocate to the created account */
      space: 8,
      /** Public key of the program to assign as the owner of the created account */
      programId: programId,
    }),
  ];

  const txn = new Transaction({
    recentBlockhash: svm.latestBlockhash(),
  }).add(...ixn);
  txn.feePayer = signer.publicKey;
  txn.sign(signer, dataAccount);
  svm.sendTransaction(txn);

  const account = svm.getAccount(dataAccount.publicKey);

  console.log(`the Data account is ${account}`);
  console.log(`DataAccountpublickey1 ${dataAccount.publicKey}`);

  let balanceDataAccount = account?.lamports;
  expect(balanceDataAccount).toBe(LAMPORTS_PER_SOL * 1.02);

  // Make the cpi to inc the No in the dataAccount
  //How to put the value in the data accountdouble.so
  //transaction to put the value

  const data = borsh.serialize(CountStruct.schema, { no: 10 });
  svm.setAccount(dataAccount.publicKey, {
    executable: false,
    /** Identifier of the program that owns the account */
    owner: programId,
    /** Number of lamports assigned to the account */
    lamports: 1030000000,
    /** Optional data assigned to the account */
    data: data,
  });
  console.log(`DataAccountpublickey ${dataAccount.publicKey}`);

  const dataAccount2 = svm.getAccount(dataAccount.publicKey);

  if (!dataAccount2) {
    throw new Error("dataAccount2 is not existAccount ");
  }

  const data2 = borsh.deserialize(CountStruct.schema, dataAccount2?.data);
  if (!data2) {
    throw new Error("Data2 is empyt");
  }
  expect(data2.no).toEqual(10);

  // cpi to double constact

  let ixn2 = TransactionInstruction({
   keys: Array<AccountMeta>;
    /**
     * Program Id to execute
     */
    programId: PublicKey;
    /**
     * Program input
     */
    data: Buffer;});
});
