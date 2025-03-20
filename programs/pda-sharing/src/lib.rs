use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("GNAx7Bnrdd8xVkd4ZzHeCeQutpAA39yreNAghryf4WqK");

#[program]
pub mod pda_sharing {
    use super::*;

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_account.to_account_info(),
            to: ctx.accounts.withdraw_destination.to_account_info(),
            authority: ctx.accounts.vault_authority.clone(),
        };
        let seeds = &[ctx.accounts.vault_mint.key.as_ref()];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    /// CHECK: The vault authority is a PDA derived solely from the vault mint's key.
    #[account(seeds = [vault_mint.key().as_ref()], bump)]
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: The vault authority is a PDA derived solely from the vault mint's key.
    pub vault_mint: AccountInfo<'info>,
    #[account(mut, constraint = withdraw_destination.owner == token_program.key())]
    pub withdraw_destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
