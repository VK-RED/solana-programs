use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount,MintTo,mint_to},
    metadata::{
        Metadata as Metaplex,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
        create_metadata_accounts_v3,
    },
    associated_token::AssociatedToken,
};

declare_id!("BzJ7zpfF3dayE6YUhtEtNowZHoZ4NmCRe3emVhjjNoa1");

#[program]
pub mod token_spl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeTokenParams) -> Result<()> {

        let accounts: &mut Initialize<'_> = ctx.accounts;

        // token metadata
        let token_metadata: DataV2 = DataV2 {
            name:params.name,
            symbol: params.symbol,
            uri: params.uri,
            collection:None,
            creators:None,
            seller_fee_basis_points: 0,
            uses:None,
        };

        // Accounts involved in create_metadata_accounts_v3 instruction
        let create_metadata_ins_accounts: CreateMetadataAccountsV3<'_> = CreateMetadataAccountsV3{
            mint: accounts.mint.to_account_info(),
            mint_authority: accounts.payer.to_account_info(),
            update_authority: accounts.payer.to_account_info(),
            metadata: accounts.metadata_account.to_account_info(),
            payer: accounts.payer.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
            rent: accounts.rent.to_account_info(),
        };

        let metadata_ctx: CpiContext<'_, '_, '_, '_, CreateMetadataAccountsV3<'_>> = CpiContext::new(accounts.metadata_program.to_account_info(), create_metadata_ins_accounts);

        create_metadata_accounts_v3(metadata_ctx, token_metadata, false, true, None)?;

        msg!("Mint Initialized Successfully!");
        Ok(())
    }

    pub fn initialize_ata(_ctx:Context<InitializeAta>) -> Result<()> {
            
        // This doesnot work :( we have to depend on the TokenAccount macro

        // let accounts: &mut InitializeAta<'_> = ctx.accounts;

        // let cpi_accounts = Create {
        //     associated_token: accounts.associated_token_program.to_account_info(),
        //     authority: accounts.payer.to_account_info(),
        //     mint: accounts.mint.to_account_info(),
        //     payer: accounts.payer.to_account_info(),
        //     system_program: accounts.system_program.to_account_info(),
        //     token_program: accounts.token_program.to_account_info(),
        // };

        // let cpi_context: CpiContext<'_, '_, '_, '_, Create<'_>> = CpiContext::new(accounts.associated_token_program.to_account_info(), cpi_accounts);

        // create(cpi_context)?;

        Ok(())
    }

    pub fn mint_tokens(ctx:Context<MintTokens>, amount: u64) -> Result<()> {
        
        let accounts = ctx.accounts;

        let mint_to_accounts = MintTo {
            authority: accounts.payer.to_account_info(),
            mint: accounts.mint.to_account_info(),
            to: accounts.token_account.to_account_info(),
        };

        let mint_to_ctx = CpiContext::new(accounts.token_program.to_account_info(), mint_to_accounts);

        mint_to(mint_to_ctx, amount)?;

        Ok(())
    }

}


/*
    User can create a new mint with metadata
    init ata
    mint tokens 
    and transfer tokens
*/

#[derive(Accounts)]
pub struct MintTokens<'info>{
    #[account(mut)]
    payer:Signer<'info>,

    #[account(
        mut,
        mint::authority = payer,
    )]
    mint: Account<'info, Mint>,

    #[account(
        mut,
        // associated_token::mint = mint,     
        // associated_token::authority = payer,
    )]
    token_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitializeAta<'info>{

    #[account(mut)]
    payer: Signer<'info>,
    mint : Account<'info, Mint>,

    #[account(
        init,
        payer=payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    associated_token_account: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeTokenParams{
    name: String,
    symbol: String,
    uri: String,
    decimals: u8
}

#[derive(Accounts)]
#[instruction(data:InitializeTokenParams)]
pub struct Initialize<'info>{
    #[account(mut)]
    payer: Signer<'info>,

    // as we used init, the mint account will be created by token program behind the scenes
    #[account(
        init,
        payer=payer,
        mint::decimals = data.decimals,
        mint::authority = payer
    )]
    mint : Account<'info, Mint>,

    //NOTE: There is no init here as we will be initializing in the instruction
    #[account(mut)]
    /// CHECK: The metadata account will be created by Metadata program,
    metadata_account: UncheckedAccount<'info>,

    rent: Sysvar<'info, Rent>,

    metadata_program: Program<'info, Metaplex>,
    system_program: Program<'info, System>,
    token_program : Program<'info, Token>
}
