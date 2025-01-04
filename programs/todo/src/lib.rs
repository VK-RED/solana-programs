use anchor_lang::prelude::*;

declare_id!("GKadSMTY4efrpHQy2NRo6UXYr7LhHKXMUArap2agdeES");

#[program]
pub mod todo {
    use super::*;

    pub fn initialize(ctx:Context<Initialize>) -> Result<()> {
        let account: &mut Account<'_, TodosAccount> = &mut ctx.accounts.todos;
        account.todos = Vec::new();
        account.bump = ctx.bumps.todos;
        Ok(())
    }

    pub fn add_todo(ctx:Context<AddTodo>, data: Todo) -> Result<()> {

        let title = &data.title;
        let description = &data.description;

        require!(title.bytes().len() <= 15, TodoError::TitleLimit);

        if let Some(val) = description {
            require!(val.bytes().len() <= 100, TodoError::DescriptionLimit);
        }
    
        let todos: &mut Vec<Option<Todo>> = &mut ctx.accounts.todos_account.todos;

        let mut add_todo_ind: usize = 0;

        let mut limit_reached: bool = true;
        
        for (ind, todo) in todos.iter().enumerate() {

            if let None = todo {
                add_todo_ind = ind;
                limit_reached = false;
                break;
            }
        }

        require!(!limit_reached, TodoError::ReachedLimit);

        todos.insert(add_todo_ind , Some(data));


        Ok(())
    }
    
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize, InitSpace)]
pub struct Todo{
    #[max_len(15)]
    title: String, 

    #[max_len(100)]
    description: Option<String>,

    done: bool,
}

#[account]
#[derive(InitSpace)]
pub struct TodosAccount {
    #[max_len(5)]
    todos: Vec<Option<Todo>>,
    bump: u8, 
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

#[error_code]
pub enum TodoError{
    #[msg("You have reached the maximum limit")]
    ReachedLimit,

    #[msg("Title length has to be less than or equal to 15")]
    TitleLimit,

    #[msg("Description length has to be less than or equal to 100")]
    DescriptionLimit

}