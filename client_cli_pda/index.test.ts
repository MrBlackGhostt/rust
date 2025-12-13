import { test, expect, beforeAll, describe } from "bun:test";
import { LiteSVM } from "litesvm";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

describe("Create Pda for client", () => {
  let litesvm: LiteSVM;
  let pda: PublicKey;
  let user: Keypair;
  let bump: number;
  let programId: PublicKey;

  beforeAll(() => {
    litesvm = new LiteSVM();
    programId = PublicKey.unique();
    user = new Keypair();
    pda = PublicKey.unique();
    litesvm.airdrop(user.publicKey, BigInt(2 * LAMPORTS_PER_SOL));
    litesvm.addProgramFromFile(programId, "./cpi.so");
    [pda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("client1"), user.publicKey.toBuffer()],
      programId,
    );

    let ix = new TransactionInstruction({
      programId: programId,
      keys: [
        {
          pubkey: user.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      data: Buffer.from([]),
    });

    const tx = new Transaction().add(ix);

    tx.feePayer = user.publicKey;
    tx.recentBlockhash = litesvm.latestBlockhash();
    tx.sign(user);
    let res = litesvm.sendTransaction(tx);
    console.log(res.toString());
  });
  test("should create pda", () => {
    const balance = litesvm.getBalance(pda);
    console.log(balance);
    expect(Number(balance)).toBeGreaterThan(0);
    expect(Number(balance)).toBe(1000000000);
  });
});
