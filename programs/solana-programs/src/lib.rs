use anchor_lang::prelude::*;

declare_id!("HQ6rZu4pav6Cp5Dhbyho9NhDWVEpTcQR5Lyy16VGw9qj");

#[program]
pub mod solana_programs {
    use super::*;

    pub fn initialize(ctx:Context<Initialize>, data:u64) -> Result<()>{
        ctx.accounts.my_account.counter = data;
        msg!("Set the value : {} for counter", data);
        Ok(())
    }

    pub fn increment(ctx:Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter+=1;
        Ok(())
    }

    pub fn decrement(ctx:Context<Decrement>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        
        
        if counter_account.counter == 0 {
            return err!(MyError::CounterNegative);
        }

        counter_account.counter-=1;
        Ok(())
    }

}

#[account]
struct CounterAccount{
    counter: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info>{   

    #[account(init, payer = authority, space = 8 + 8)]
    my_account : Account<'info, CounterAccount>,

    #[account(signer,mut)]
    authority: Signer<'info>,

    system_program : Program<'info, System>
}   


#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    counter_account : Account<'info, CounterAccount>
}

#[derive(Accounts)]
pub struct Decrement<'info>{
    #[account(mut)]
    counter_account : Account<'info, CounterAccount>
}

#[error_code]
pub enum MyError{
    #[msg("Counter Value cannot be negative")]
    CounterNegative
}