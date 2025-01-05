use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod errors;
pub mod state;
pub mod instructions;

declare_id!("GKadSMTY4efrpHQy2NRo6UXYr7LhHKXMUArap2agdeES");

#[program]
pub mod todo {
    use super::*;

    pub fn initialize(ctx:Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn add_todo(ctx:Context<AddTodo>, data: Todo) -> Result<()> {
        instructions::add_todo(ctx, data)
    }
}