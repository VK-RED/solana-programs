use anchor_lang::prelude::*;
use crate::state::{TodosAccount,Todo};
use crate::errors::TodoError;

pub fn add_todo(ctx:Context<AddTodo>, data: Todo) -> Result<()> {

    let title = &data.title;
    let description = &data.description;

    require!(title.bytes().len() <= 15, TodoError::TitleLimit);

    if let Some(val) = description {
        require!(val.bytes().len() <= 100, TodoError::DescriptionLimit);
    }

    let todos_account:&mut Account<'_, TodosAccount>  = &mut ctx.accounts.todos_account;
    todos_account.add_todo(data)
}

#[derive(Accounts)]
pub struct AddTodo<'info>{
    user : Signer<'info>,

    #[account(
        mut,
        seeds = [b"todos", user.key().as_ref()],
        bump = todos_account.bump,
    )]
    todos_account : Account<'info, TodosAccount>,
}