use anchor_lang::prelude::*;
use crate::state::TodosAccount;


pub fn initialize(ctx:Context<Initialize>) -> Result<()> {
    let account: &mut Account<'_, TodosAccount> = &mut ctx.accounts.todos;
    account.todos = vec![None; 5];
    account.bump = ctx.bumps.todos;
    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user : Signer<'info>,

    #[account(
        init,
        payer = user,
        space = TodosAccount::INIT_SPACE,
        seeds = [b"todos", user.key().as_ref()],
        bump
    )]
    todos: Account<'info, TodosAccount>,
    system_program: Program<'info, System>,
}