use anchor_lang::prelude::*;

declare_id!("GNAx7Bnrdd8xVkd4ZzHeCeQutpAA39yreNAghryf4WqK");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn update_admin(ctx: Context<UpdateAdmin>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.admin_state.admin = new_admin;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub admin_state: Account<'info, AdminState>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct AdminState {
    pub admin: Pubkey,
}
