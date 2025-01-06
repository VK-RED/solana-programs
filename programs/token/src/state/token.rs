use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenAccount {
    pub mint_authority : Option<Pubkey>,
    #[max_len(25)]
    pub name : String,
    #[max_len(5)]
    pub symbol: String,
    pub decimal: u8,
    pub supply: u64
}

impl TokenAccount{
    pub fn increase_supply(&mut self, amount:u64){
        self.supply += amount;
    }

    pub fn revoke_mint_authority(&mut self){
        self.mint_authority = None;
    }
}