use anchor_lang::prelude::*;

use crate::{errors::TokenErrors, state::{Ata, TokenAccount}};

pub fn send_tokens(ctx:Context<SendTokens>, amount: u64) -> Result<()> {

    require!(amount > 10, TokenErrors::TokensTooLow); //For now restrict sending below 10 tokens 
    let payer = &mut ctx.accounts.payer_ata;
    let payee = &mut ctx.accounts.payee_ata;
    payer.send_tokens(payee, amount);
    Ok(())
}


#[derive(Accounts)]
#[instruction(amount:u64)]
pub struct SendTokens<'info>{
    payer: Signer<'info>,

    #[account(
        mut,
        constraint = payer_ata.authority.key().as_ref() == payer.key().as_ref(),
        constraint = payer_ata.balance >= amount @ TokenErrors::InsufficientFunds, // amount has to be GOE to the balance
        seeds = [b"ata", payer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    payer_ata: Account<'info, Ata>,

    #[account(
        mut,
        owner = crate::ID,
    )]
    payee_ata:Account<'info, Ata>,

    mint: Account<'info, TokenAccount>,
}