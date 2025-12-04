import * as borsh from "borsh";
import { expect, test } from "bun:test";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { GREETING_SIZE, schema, numberMath } from "./type";

const userKey = Keypair.generate();
const noKeypair = Keypair.generate();

test("number does increase", async () => {
  const connection = new Connection("http://localhost:8899", "confirmed");
  console.log(`User key is ${userKey}`);
  const res = await connection.requestAirdrop(
    userKey.publicKey,
    2 * LAMPORTS_PER_SOL,
  );

  await connection.confirmTransaction(res);

  const programID = new PublicKey(
    "9ARnwUoJzkW6f16HBUcmPUJbQ5YH3S4osXGK8m5LqdnB",
  );

  const lampport =
    await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

  const numberAccount = SystemProgram.createAccount({
    fromPubkey: userKey.publicKey,
    lamports: lampport,
    newAccountPubkey: noKeypair.publicKey,
    programId: programID,
    space: GREETING_SIZE,
  });

  const tx = new Transaction();
  tx.add(numberAccount);

  const txHash = await connection.sendTransaction(tx, [userKey, noKeypair]);

  await connection.confirmTransaction(txHash);

  const noAccount = await connection.getAccountInfo(noKeypair.publicKey);

  if (!noAccount) {
    throw new Error("Counter account not found");
  }
  const num = borsh.deserialize(schema, noAccount.data) as numberMath;

  console.log(num);
  expect(num?.no).toBe(0);
});
