use anchor_lang::prelude::*;
use crate::state::TokenAccount;
use crate::errors::TokenErrors;

pub const DISCRIMINATOR_SIZE : usize = 8;

pub fn initialize(ctx:Context<Initialize>, data:InitializeData)->Result<()>{

    let token_account = &mut ctx.accounts.token;

    require!(token_account.name.bytes().len() <= 25, TokenErrors::NameTooLarge);
    require!(token_account.symbol.bytes().len() <= 5, TokenErrors::SymbolTooLarge);

    const DECIMAL:u8 = 9;
    token_account.decimal = data.decimal.unwrap_or(DECIMAL);
    token_account.name = data.name;
    token_account.symbol = data.symbol;
    token_account.supply = 0;
    token_account.mint_authority = Some(ctx.accounts.user.key());
    
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init,
        space = DISCRIMINATOR_SIZE + TokenAccount::INIT_SPACE,
        payer = user
    )]
    token: Account<'info, TokenAccount>,
    system_program : Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeData{
    name : String,
    symbol: String,
    decimal: Option<u8>
}