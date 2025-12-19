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

// enum CountInstruction {
//     Init(u32),  // Number 0
//     Double,     // Number 1  ← You want this!
//     Half,       // Number 2
// }
//
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
  const cpiProgramId = PublicKey.unique();
  svm.airdrop(signer.publicKey, BigInt(LAMPORTS_PER_SOL * 2));

  svm.addProgramFromFile(programId, "./double.so");
  svm.addProgramFromFile(cpiProgramId, "./cli.so");

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

  svm.expireBlockhash();
  const account = svm.getAccount(dataAccount.publicKey);

  console.log(`the Data account is ${account}`);
  console.log(`DataAccountpublickey1 ${dataAccount.publicKey}`);

  let balanceDataAccount = account?.lamports;
  expect(balanceDataAccount).toBe(LAMPORTS_PER_SOL * 1.02);

  //transaction to put the value

  const data = borsh.serialize(CountStruct.schema, { no: 10 });

  //Putting the data to the dataAccount
  svm.setAccount(dataAccount.publicKey, {
    executable: false,
    owner: programId,
    lamports: 1030000000,
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

  // Call the Double instructions in the double constact
  let doubleEnumInstructionData = Buffer.from([1]);
  let ixn2 = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAccount.publicKey,
        isSigner: false,
        isWritable: true,
      },
    ], // Array<AccountMeta>
    /**
     * Program Id to execute
     */
    programId: programId,
    data: doubleEnumInstructionData,
  });

  const transactionToDoubleContract = new Transaction({
    recentBlockhash: svm.latestBlockhash(),
  }).add(ixn2);

  transactionToDoubleContract.feePayer = signer.publicKey;

  transactionToDoubleContract.sign(signer, dataAccount);
  svm.sendTransaction(transactionToDoubleContract);

  // ✅ Check the result
  const dataAccount3 = svm.getAccount(dataAccount.publicKey);
  if (!dataAccount3) {
    throw new Error("dataAccount3 is not present");
  }
  const data3 = borsh.deserialize(CountStruct.schema, dataAccount3?.data);
  if (!data3 || !data3.no) {
    throw new Error("data is not there");
  }
  console.log("After Double:", data3.no); // Should print 20
  expect(data3.no).toEqual(20);

  // Cpi to the transactionToDoubleContract

  const cpiToDoubleInx = Buffer.from([1]);
  const cpiInstruction = new TransactionInstruction({
    keys: [
      { pubkey: signer.publicKey, isSigner: true, isWritable: false },
      {
        pubkey: dataAccount.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: programId,
        isSigner: false,
        isWritable: true,
      },
    ],
    programId: cpiProgramId,
    data: cpiToDoubleInx,
  });
  svm.expireBlockhash();
  const cpiTransaction = new Transaction({
    recentBlockhash: svm.latestBlockhash(),
  }).add(cpiInstruction);

  cpiTransaction.feePayer = signer.publicKey;
  cpiTransaction.sign(signer);
  // ✅ FIX 1: Send the transaction!
  const cpiResult = svm.sendTransaction(cpiTransaction);
  console.log(`the transaction ${cpiResult}`);

  // ✅ Check the result
  const dataAccount4 = svm.getAccount(dataAccount.publicKey);
  if (!dataAccount4) {
    throw new Error("dataAccount3 is not present");
  }
  const data4 = borsh.deserialize(CountStruct.schema, dataAccount4?.data);
  if (!data4 || !data4.no) {
    throw new Error("data is not there");
  }
  console.log("After CPIDouble:", data4.no); // Should print 20
  expect(data4.no).toEqual(40);
});
