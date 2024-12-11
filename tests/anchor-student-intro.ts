import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntro } from "../target/types/anchor_student_intro";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token"

describe("anchor-student-intro", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorStudentIntro as Program<AnchorStudentIntro>;

  const intro = {
    name: "Ivan",
    message: "Hello students!",
  }
  const reallocIntro = {
    name: "Ivan",
    message: "Hello students! realloc",
  }

  const [introPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(intro.name), provider.wallet.publicKey.toBuffer()],
    program.programId,
  );

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  it("Initializes the reward token", async () => {
    const tx = await program.methods.initializeTokenMint().rpc();
  });
  it("Student Intro added", async () => {
    // Add your test here.
    const tokenAccount = await getAssociatedTokenAddress(
      mint,
      provider.wallet.publicKey,
    );
    const tx = await program.methods.addStudentIntro(intro.name, intro.message).rpc();
    const account = await program.account.studentAccountState.fetch(introPda);
    console.log("Your transaction signature", tx);
    expect(intro.name).equal(account.name);
    expect(intro.message).equal(account.message);
    expect(account.reviewer.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58(),
    );

    const userAta = await getAccount(provider.connection, tokenAccount);
    expect(Number(userAta.amount)).to.equal((10 * 10) ^ 6);
  });
  it("Student Intro updated", async () => {
    // Add your test here.
    const tx = await program.methods.updateStudentIntro(reallocIntro.name, reallocIntro.message).rpc();
    const account = await program.account.studentAccountState.fetch(introPda);
    console.log("Your transaction signature", tx);
    expect(reallocIntro.name).equal(account.name);
    expect(reallocIntro.message).equal(account.message);
  });
  it("Student Intro deleted", async () => {
    // Add your test here.
    const tx = await program.methods.deleteStudentIntro(intro.name).rpc();
    console.log("Your transaction signature", tx);
  });

});
