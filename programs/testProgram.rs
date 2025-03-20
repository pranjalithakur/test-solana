use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgpqDZzk8");

#[program]
pub mod vulnerable_program {
    use super::*;

    // A deposit instruction that increases the user's balance.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.balance = user_account.balance.checked_add(amount).unwrap();
        Ok(())
    }

    // A withdraw instruction that decreases the user's balance.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.balance = user_account.balance.checked_sub(amount).unwrap();
        Ok(())
    }
}

#[account]
pub struct UserAccount {
    // Balance is stored as an unsigned 64-bit integer.
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    // The user's account must be mutable to update the balance.
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // The user's account must be mutable to update the balance.
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}
