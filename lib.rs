use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("7NfwhxtCKALKsspUkVj52SkqLMEBdNF97icvYfnHviuu");

#[program] //place where all the functions are defined 
mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter_account.count=0;
        msg!("Initialized counter to : {}", ctx.accounts.counter_account.count); // Message will show up in the tx logs
        Ok(())
    }
    pub fn increment(ctx:Context<Change>)->Result<()>{
        ctx.accounts.counter_account.count+=1;
        msg!("Increment Operation");
        Ok(())
    }
    pub fn decrement(ctx:Context<Change>)->Result<()>{
        ctx.accounts.counter_account.count-=1;
        msg!("Increment Operation");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 8)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Change<'info>{
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>
}

#[account]
pub struct CounterAccount {
    pub count: u64
}
