use anchor_lang::prelude::*;

declare_id!("GNAx7Bnrdd8xVkd4ZzHeCeQutpAA39yreNAghryf4WqK");

#[program]
pub mod improper_account_closing {
    use super::*;

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        let account_to_close = &mut ctx.accounts.account_to_close;
        let destination = &mut ctx.accounts.destination;

        let lamports_to_transfer = **account_to_close.lamports.borrow();

        **destination.lamports.borrow_mut() += lamports_to_transfer;

        **account_to_close.lamports.borrow_mut() = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    /// CHECK: This account is the one to be closed. We do not perform any additional checks on it,
    #[account(mut)]
    pub account_to_close: AccountInfo<'info>,
    /// CHECK: This is the destination account that receives the lamports. No additional checks are necessary.
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}
