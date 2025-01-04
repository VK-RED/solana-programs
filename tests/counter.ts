import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";

describe("counter program", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaPrograms as Program<Counter>;

  const authority = provider.wallet as anchor.Wallet;
  const newAccount = Keypair.generate();

  it("Is initialized!", async () => {

    const tx = await program.methods
    .initialize(new anchor.BN(1))
    .accounts({
      authority: authority.publicKey,
      myAccount: newAccount.publicKey,
    })
    .signers([newAccount])
    .rpc();

    console.log("The tx is : ", tx);

    const counterAddress = await program.account.counterAccount.fetch(newAccount.publicKey)    
    
    expect(counterAddress.counter.toNumber()).equal(1);

  });

  it("Should increment the counter value", async() => {

    await program.methods
    .increment()
    .accounts({
      counterAccount: newAccount.publicKey,
    })
    .rpc()
    const counterAccount = await program.account.counterAccount.fetch(newAccount.publicKey)
    expect(counterAccount.counter.toNumber()).equals(2)

  })

  it("Should decrement the counter value", async() => {

    await program.methods
    .decrement()
    .accounts({
      counterAccount: newAccount.publicKey,
    })
    .rpc();

    const counterAccount = await program.account.counterAccount.fetch(newAccount.publicKey);

    console.log("The Counter Value : ", counterAccount.counter);

    expect(counterAccount.counter.toNumber()).equals(1)

  })
  
  it("should throw error on decrement when the value is Zero", async () => {

    const newAccount = Keypair.generate();

    await program.methods
    .initialize(new anchor.BN(0))
    .accounts({
      authority: authority.publicKey,
      myAccount: newAccount.publicKey,
    })
    .signers([newAccount])
    .rpc();

    const account = await program.account.counterAccount.fetch(newAccount.publicKey);

    expect(account.counter.toNumber()).equals(0);

    try {

      await program.methods
      .decrement()
      .accounts({
        counterAccount: newAccount.publicKey,
      })
      .rpc();

      assert(false, "it should not have reached here");

    } catch (_err) {
      expect(_err).to.be.instanceOf(anchor.AnchorError);
      const err : anchor.AnchorError = _err;
      expect(err.error.errorCode.code).to.equal("CounterNegative");
      expect(err.program.equals(program.programId));
    }

  })

});
