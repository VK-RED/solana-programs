use anchor_lang::prelude::*;
use crate::state::{Ata, TokenAccount};

pub fn initialize_ata(ctx:Context<InitializeAta>) -> Result<()> {

    let ata: &mut Account<'_, Ata> = &mut ctx.accounts.ata;
    ata.mint = ctx.accounts.mint.key();
    ata.authority = ctx.accounts.authority.key();
    ata.balance = 0;
    Ok(())
}


#[derive(Accounts)]
pub struct InitializeAta<'info>{
    authority: Signer<'info>,

    #[account(mut)]
    payer: Signer<'info>,
    
    mint: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + Ata::MAX_SIZE,
        seeds = [b"ata", authority.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    ata: Account<'info, Ata>,

    system_program: Program<'info, System>
}