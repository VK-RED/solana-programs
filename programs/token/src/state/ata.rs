use anchor_lang::prelude::*;

#[account]
pub struct Ata{
    pub mint : Pubkey, //32
    pub balance : u64, //8
    pub authority : Pubkey, //32
}

impl Ata {
    pub const MAX_SIZE : usize = 32 + 32 + 8;

    pub fn increase_balance(&mut self, amount:u64){
        self.balance+= amount;
    }

    pub fn send_tokens(&mut self, payee:&mut Ata, amount:u64){
        self.balance -= amount;
        payee.balance += amount;
    }
}

