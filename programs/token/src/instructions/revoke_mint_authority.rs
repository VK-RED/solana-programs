use anchor_lang::prelude::*;

use crate::{errors::TokenErrors, state::TokenAccount};

pub fn revoke_mint_authority(ctx:Context<RevokeMintAuthority>)->Result<()>{
    let mint: &mut Account<'_, TokenAccount> = &mut ctx.accounts.mint;
    mint.revoke_mint_authority();
    Ok(())
}


#[derive(Accounts)]
pub struct RevokeMintAuthority<'info>{
    #[account(
        mut,
        constraint = mint.mint_authority.unwrap().key().as_ref() == mint_authority.key().as_ref() @ TokenErrors::PermissionDenied
    )]
    mint: Account<'info, TokenAccount>,
    mint_authority: Signer<'info>,
}