import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Token} from "../target/types/token";
import {Keypair, PublicKey} from "@solana/web3.js";
import { assert, expect } from "chai";

describe("Token program", () => {

    const program = anchor.workspace.Token as Program<Token>;
    const PROGRAM_ID = program.programId;

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const keypair = anchor.Wallet.local().payer;
    const token = anchor.web3.Keypair.generate();

    it("Should initialize a new mint", async() => {

        await program.methods
        .initialize({
            name:"DADDY TATE",
            symbol:"DADTA",
            decimal:null
        })
        .accounts({
            user:keypair.publicKey,
            token: token.publicKey,
        })
        .signers([
            token,
        ])
        .rpc();

        const mintAccount = await program.account.tokenAccount.fetch(token.publicKey);

        expect(mintAccount.name).to.be.equal("DADDY TATE");
        expect(mintAccount.decimal).to.be.equal(9);
        expect(mintAccount.supply.toNumber()).to.be.equal(0);
        expect(mintAccount.symbol).to.be.equal("DADTA");
        expect(mintAccount.mintAuthority.toBase58).to.be.equal(keypair.publicKey.toBase58);

    })

    it("Should initialize a new ATA", async() => {

        const tx = await program.methods
        .initializeAta()
        .accounts({
            mint:token.publicKey,
            authority: keypair.publicKey,
            payer: keypair.publicKey,
        })
        .rpc();

        const PDA = getPDA(keypair.publicKey, token.publicKey, PROGRAM_ID);

        const ata = await program.account.ata.fetch(PDA);

        expect(ata.authority.toBase58()).to.be.equal(keypair.publicKey.toBase58());
        expect(ata.mint.toBase58()).to.be.equal(token.publicKey.toBase58());
        expect(ata.balance.toNumber()).to.be.equal(0);
    })

    it("Should mint tokens to a given ATA", async() => {

        const PDA = getPDA(keypair.publicKey, token.publicKey, program.programId);
        
        const amount = new anchor.BN(1000000000)

        await program.methods
        .mintToken(amount)
        .accounts({
            ata:PDA,
            mintAuthority: keypair.publicKey
        })
        .signers([keypair])
        .rpc()

        const mint = await program.account.tokenAccount.fetch(token.publicKey);
        const ata =  await program.account.ata.fetch(PDA);

        expect(mint.supply.toNumber()).to.be.equal(amount.toNumber());
        expect(ata.balance.toNumber()).to.be.equal(amount.toNumber());


    })

    it("Should throw an error when non-mint-authority passed to mint tokens", async() => {

        try {

            const nonMintAuthority = Keypair.generate();

            const PDA = getPDA(keypair.publicKey, token.publicKey, program.programId);
            
            const amount = new anchor.BN(1000000000)

            await program.methods
            .mintToken(amount)
            .accounts({
                ata:PDA,
                mintAuthority: nonMintAuthority.publicKey
            })
            .signers([nonMintAuthority])
            .rpc()

            assert(false, "it should not have reached here");
            
        } catch (error) {
            if(error instanceof anchor.AnchorError){
                expect(error.error.errorCode.code).to.be.equal("RequireKeysEqViolated")
            }
            else{
                throw new Error(error.message);
            }

        }
    })

    it("Should revoke mint authority", async() => {
        await program.methods
        .revokeMintAuthority()
        .accounts({
            mint:token.publicKey,
        })
        .signers([keypair])
        .rpc();

        const mint = await program.account.tokenAccount.fetch(token.publicKey);
        expect(mint.mintAuthority).to.be.equal(null);
    })

    it("Should throw an error upon minting when mint authority is revoked", async() => {
        try {

            const PDA = getPDA(keypair.publicKey, token.publicKey, program.programId);
        
            const amount = new anchor.BN(1000000000)

            await program.methods
            .mintToken(amount)
            .accounts({
                ata:PDA,
                mintAuthority: keypair.publicKey
            })
            .signers([keypair])
            .rpc()

            assert(false, "it should not have reached here");
            
        } catch (error) {
            if(error instanceof anchor.AnchorError){
                const err : anchor.AnchorError = error;
                expect(err.error.errorCode.code).to.be.equal("MintAccessRevoked")
            }
            else{
                throw new Error(error.message);
            }
        }
    })

    it("Should send tokens from one account to the other", async() => {

        const payee = Keypair.generate();

        const payeeAta = getPDA(payee.publicKey, token.publicKey, program.programId);
        const payerAta = getPDA(keypair.publicKey, token.publicKey, program.programId);
        
        await program.methods
        .initializeAta()
        .accounts({
            mint:token.publicKey,
            authority: payee.publicKey,
            payer: keypair.publicKey,
        })
        .signers([keypair, payee])
        .rpc()

        await program.methods
        .sendTokens(new anchor.BN(1000))
        .accounts({
            mint: token.publicKey,
            payer: keypair.publicKey,
            payeeAta
        })
        .signers([keypair])
        .rpc()

        const payerAccount = await program.account.ata.fetch(payerAta);
        const payeeAccount = await program.account.ata.fetch(payeeAta);

        expect(payerAccount.balance.toNumber()).to.be.equal(999999000);
        expect(payeeAccount.balance.toNumber()).to.be.equal(1000);

    })

    it("Should throw an error upon insufficient funds", async ()=>{

        try {

            const payee = Keypair.generate();

            const payeeAta = getPDA(payee.publicKey, token.publicKey, program.programId);
            
            await program.methods
            .initializeAta()
            .accounts({
                mint:token.publicKey,
                authority: payee.publicKey,
                payer: keypair.publicKey,
            })
            .signers([keypair, payee])
            .rpc()
    
            await program.methods
            .sendTokens(new anchor.BN(1000000000))
            .accounts({
                mint: token.publicKey,
                payer: keypair.publicKey,
                payeeAta
            })
            .signers([keypair])
            .rpc()

            assert(false, "it should not have reached here !");
            
        } catch (error) {
            if(error instanceof anchor.AnchorError){
                const err = error
                expect(err.error.errorCode.code).to.be.equal("InsufficientFunds");
            }
        }

    })
})


const getPDA = (authority:PublicKey, mintAddress:PublicKey, PROGRAM_ID: PublicKey) => {
    const [PDA] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("ata"),
        authority.toBuffer(),
        mintAddress.toBuffer(),
    ],PROGRAM_ID);

    return PDA;
}
