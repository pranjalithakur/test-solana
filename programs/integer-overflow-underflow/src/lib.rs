use anchor_lang::prelude::*;

declare_id!("GNAx7Bnrdd8xVkd4ZzHeCeQutpAA39yreNAghryf4WqK");

#[program]
pub mod integer_overflow_underflow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let admin_state = &mut ctx.accounts.admin_account;
        admin_state.admin = *ctx.accounts.authority.key;
        msg!("Initialized admin: {:?}", ctx.accounts.authority.key);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let admin_state = &mut ctx.accounts.admin_account;
        admin_state.counter += 1;  
        msg!("Counter incremented to: {}", admin_state.counter);
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let admin_state = &mut ctx.accounts.admin_account;
        admin_state.counter -= 1;  
        msg!("Counter decremented to: {}", admin_state.counter);
        Ok(())
    }
}

#[account]
#[derive(Default, Debug)]
pub struct AdminState {
    pub admin: Pubkey,
    pub counter: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1)]
    pub admin_account: Account<'info, AdminState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub admin_account: Account<'info, AdminState>,
    #[account(mut, constraint = authority.key() == admin_account.admin)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub admin_account: Account<'info, AdminState>,
    #[account(mut, constraint = authority.key() == admin_account.admin)]
    pub authority: Signer<'info>,
}
