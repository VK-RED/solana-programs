use anchor_lang::prelude::*;

use crate::state::{TokenAccount,Ata};
use crate::errors::TokenErrors;

pub fn mint_token(ctx:Context<MintToken>, amount:u64)->Result<()>{

    match &ctx.accounts.mint.mint_authority {
        Some(authority) => {
            require_keys_eq!(*authority, ctx.accounts.mint_authority.key())
        },
        _ => {
            return err!(TokenErrors::MintAccessRevoked);
        } 
    };

    let ata: &mut Account<'_, Ata> = &mut ctx.accounts.ata;
    let mint: &mut Account<'_, TokenAccount> = &mut ctx.accounts.mint;

    ata.increase_balance(amount);
    mint.increase_supply(amount);

    Ok(())
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    mint_authority: Signer<'info>,
    #[account(
        mut,
    )]
    mint: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = mint @ TokenErrors::InvalidMintAccount,
    )]
    ata: Account<'info, Ata>,
}