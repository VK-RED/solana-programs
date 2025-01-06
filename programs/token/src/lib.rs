use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod state;
pub mod instructions;

declare_id!("8TuL72KoKdLURuw4VkknXNZ3bRG9yZeoxEFKRUZNWcsi");

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data:InitializeData) -> Result<()> {
        instructions::initialize(ctx, data)
    }

    pub fn initialize_ata(ctx:Context<InitializeAta>) -> Result<()> {
        instructions::initialize_ata(ctx)
    }

    pub fn mint_token(ctx:Context<MintToken>, amount:u64)->Result<()>{
        instructions::mint_token(ctx, amount)
    }

    pub fn revoke_mint_authority(ctx:Context<RevokeMintAuthority>)->Result<()>{
        instructions::revoke_mint_authority(ctx)
    }

    pub fn send_tokens(ctx:Context<SendTokens>, amount: u64) -> Result<()> {
        instructions::send_tokens(ctx, amount)
    }
}