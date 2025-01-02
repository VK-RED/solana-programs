import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaPrograms } from "../target/types/solana_programs";
import { Keypair } from "@solana/web3.js";
import fs from "fs";
import { expect } from "chai";

describe("solana-programs", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaPrograms as Program<SolanaPrograms>;

  const filePath = "/home/vishnu/.config/solana/id.json";
  const secret = JSON.parse(fs.readFileSync(filePath).toString()) as number[]; 
  const secretKey = Uint8Array.from(secret);

  const authority = anchor.web3.Keypair.fromSecretKey(secretKey)
  const newAccount = Keypair.generate();

  it("Is initialized!", async () => {

    const tx = await program.methods
    .initialize(new anchor.BN(1))
    .accounts({
      authority: authority.publicKey,
      myAccount: newAccount.publicKey,
    })
    .signers([authority,newAccount])
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
  
  //TODO : WRITE TEST FOR CATCHING NEGATIVE VALUES


});
