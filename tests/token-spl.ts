import * as anchor from "@coral-xyz/anchor";
import {Keypair, PublicKey} from "@solana/web3.js";
import { expect } from "chai";
import { TokenSpl } from "../target/types/token_spl";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

type ParsedAccountData = {
    /** Name of the program that owns this account */
    program: string;
    /** Parsed account data */
    parsed: {
        info: {
          decimals: number,
          freezeAuthority: null|string,
          isInitialized: boolean,
          mintAuthority: string,
          supply: string
        },
        type: string
      },
    /** Space used by account data */
    space: number;
};

type ParsedAtaData = {
    info: {
        isNative: false,
        mint: string,
        owner: string,
        state: string,
        tokenAmount: { 
            amount: string, 
            decimals: number, 
            uiAmount: number, 
            uiAmountString: string 
        }
    },
    type: string
}

describe("SPL Token Program", ()=>{

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const keypair = anchor.Wallet.local().payer;
    
    const program = anchor.workspace.TokenSpl as anchor.Program<TokenSpl>;
    const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    const mint = Keypair.generate();

    it("Should initialize a new mint", async() => {

        // we have to do this as metadata account should be a PDA
        const metadataAccount = anchor.web3.PublicKey.findProgramAddressSync(
            [
              Buffer.from("metadata"),
              TOKEN_METADATA_PROGRAM_ID.toBuffer(),
              mint.publicKey.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        )[0];

        await program.methods
        .initialize({
            decimals:9,
            name:"DADDY",
            symbol:"DAD",
            uri:"https://www.daddytoken.com",
        })
        .accounts({
            mint:mint.publicKey,
            payer:keypair.publicKey,
            metadataAccount: metadataAccount
        })
        .signers([keypair, mint])
        .rpc();

        const accountInfo = await provider.connection.getParsedAccountInfo(mint.publicKey);
        
        const parsedAccountData = accountInfo.value.data as ParsedAccountData;

        expect(parsedAccountData.parsed.info.mintAuthority).to.be.equal(keypair.publicKey.toBase58());
        expect(parsedAccountData.parsed.info.decimals).to.be.equal(9);
        expect(parsedAccountData.parsed.info.isInitialized).to.be.equal(true);
        expect(parsedAccountData.parsed.type).to.be.equal('mint');
    })

    it("Should Initialize a new ATA", async()=>{

        const ata = getAssociatedTokenAccount(mint.publicKey, keypair.publicKey);
        
        await program.methods
        .initializeAta()
        .accounts({
            mint:mint.publicKey,
            payer:keypair.publicKey,
        })
        .signers([keypair])
        .rpc();

        const accountInfo = await provider.connection.getParsedAccountInfo(ata);
        
        const accountData = accountInfo.value.data as anchor.web3.ParsedAccountData;
        const parsedAccountInfo = accountData.parsed as ParsedAtaData;

        expect(accountData.program).to.be.equal('spl-token');
        expect(parsedAccountInfo.info.mint).to.be.equal(mint.publicKey.toBase58());
        expect(parsedAccountInfo.info.owner).to.be.equal(keypair.publicKey.toBase58());
        expect(parsedAccountInfo.info.tokenAmount.decimals).to.be.equal(9);
        expect(parsedAccountInfo.info.tokenAmount.uiAmount).to.be.equal(0);
    })

    it("should mint tokens to a given account", async ()=> {

        const tokenLimit = new anchor.BN(1000_000_000);
        const ata = getAssociatedTokenAccount(mint.publicKey, keypair.publicKey);

        await program.methods
        .mintTokens(tokenLimit)
        .accounts({
            mint:mint.publicKey,
            payer:keypair.publicKey,
            tokenAccount:ata,
        })  
        .signers([keypair])
        .rpc()

        const accountInfo = await provider.connection.getParsedAccountInfo(ata);
        
        const accountData = accountInfo.value.data as anchor.web3.ParsedAccountData;
        const parsedAccountInfo = accountData.parsed as ParsedAtaData;
        
        expect(parsedAccountInfo.info.tokenAmount.amount).to.be.equal("1000000000");
        expect(parsedAccountInfo.info.tokenAmount.uiAmount).to.be.equal(1);

    })

    // it("should throw an error when incorrect mint authority is passed")

    // it("should throw an error when uninitialized ata is passed")

    // it("should create an ata and mint tokens")

    // it("should transfer tokens")

    // it("should throw an error when transferring upon insufficient funds")
})


const getAssociatedTokenAccount = (mintAddress:PublicKey, user:PublicKey) => {

    const [ata] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            user.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            mintAddress.toBuffer(),
        ],
        ASSOCIATED_PROGRAM_ID
    );

    return ata;
}