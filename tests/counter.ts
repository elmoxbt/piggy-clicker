import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";
import assert from "assert";

describe("counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Counter as Program<Counter>;
  const user = anchor.web3.Keypair.generate();
  let counterPDA: PublicKey;

  const getCounterAddress = (authority: PublicKey) => {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), authority.toBuffer()],
      program.programId
    );
  };

  const airdrop = async (publicKey: PublicKey) => {
    const sig = await program.provider.connection.requestAirdrop(publicKey, 1_000_000_000);
    await program.provider.connection.confirmTransaction(sig, "confirmed");
  };

  before(async () => {
    await airdrop(user.publicKey);
    [counterPDA] = getCounterAddress(user.publicKey);
  });

  it("Initializes counter", async () => {
    const maxCount = new anchor.BN(20);
    await program.methods
      .initialize(maxCount)
      .accounts({
        counter: counterPDA,
        authority: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPDA);
    assert.equal(counterAccount.count.toNumber(), 0);
    assert.equal(counterAccount.maxCount.toNumber(), 20);
    assert.equal(counterAccount.authority.toString(), user.publicKey.toString());
  });

  it("Increments counter", async () => {
    const counterBefore = await program.account.counter.fetch(counterPDA);
    await program.methods
      .increment()
      .accounts({ counter: counterPDA, authority: user.publicKey })
      .signers([user])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPDA);
    assert.equal(counterAccount.count.toNumber(), counterBefore.count.toNumber() + 1);
  });

  it("Fails to increment past max", async () => {
    for (let i = 1; i < 20; i++) {
      await program.methods
        .increment()
        .accounts({ counter: counterPDA, authority: user.publicKey })
        .signers([user])
        .rpc();
    }

    try {
      await program.methods
        .increment()
        .accounts({ counter: counterPDA, authority: user.publicKey })
        .signers([user])
        .rpc();
      assert.fail("Should fail with CountExceeded");
    } catch (err) {
      assert.ok(err.message.includes("CountExceeded"));
    }
  });

  it("Decrements counter", async () => {
    const counterBefore = await program.account.counter.fetch(counterPDA);
    await program.methods
      .decrement()
      .accounts({ counter: counterPDA, authority: user.publicKey })
      .signers([user])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPDA);
    assert.equal(counterAccount.count.toNumber(), counterBefore.count.toNumber() - 1);
  });

  it("Fails to decrement below zero", async () => {
    for (let i = 0; i < 19; i++) {
      await program.methods
        .decrement()
        .accounts({ counter: counterPDA, authority: user.publicKey })
        .signers([user])
        .rpc();
    }

    try {
      await program.methods
        .decrement()
        .accounts({ counter: counterPDA, authority: user.publicKey })
        .signers([user])
        .rpc();
      assert.fail("Should fail with Underflow");
    } catch (err) {
      assert.ok(err.message.includes("Underflow"));
    }
  });

  it("Resets counter", async () => {
    await program.methods
      .increment()
      .accounts({ counter: counterPDA, authority: user.publicKey })
      .signers([user])
      .rpc();

    await program.methods
      .reset()
      .accounts({ counter: counterPDA, authority: user.publicKey })
      .signers([user])
      .rpc();

    try {
      await program.account.counter.fetch(counterPDA);
      assert.fail("Account should be closed");
    } catch (err) {
      assert.ok(err.message.includes("Account does not exist"));
    }
  });

  it("Fails unauthorized access", async () => {
    await program.methods
      .initialize(new anchor.BN(10))
      .accounts({
        counter: counterPDA,
        authority: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const otherUser = anchor.web3.Keypair.generate();
    await airdrop(otherUser.publicKey);

    try {
  await program.methods
    .increment()
    .accounts({ counter: counterPDA, authority: otherUser.publicKey })
    .signers([otherUser])
    .rpc();
  assert.fail("Should fail with Unauthorized");
} catch (err) {
  assert.ok(err.message.includes("Unauthorized"));
}
  });
});