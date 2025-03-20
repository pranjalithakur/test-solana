use anchor_lang::prelude::*;

declare_id!("GNAx7Bnrdd8xVkd4ZzHeCeQutpAA39yreNAghryf4WqK");

#[program]
pub mod missing_signer_check {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let admin_state = &mut ctx.accounts.admin_account;
        admin_state.admin = *ctx.accounts.authority.key;
        msg!("Initialized admin: {:?}", ctx.accounts.authority.key);
        Ok(())
    }

    pub fn update_admin(ctx: Context<UpdateAdmin>) -> Result<()> {
        let admin_state = &mut ctx.accounts.admin_account;
        admin_state.admin = *ctx.accounts.new_admin.key;
        msg!("Admin updated to: {:?}", ctx.accounts.new_admin.key);
        Ok(())
    }
}

#[account]
#[derive(Default, Debug)]
pub struct AdminState {
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub admin_account: Account<'info, AdminState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub admin_account: Account<'info, AdminState>,
    /// CHECK: This account is intentionally unchecked for signature to simulate a missing signer vulnerability.
    pub authority: AccountInfo<'info>,
    /// CHECK: This account is intentionally unchecked for signature to simulate a missing signer vulnerability.
    pub new_admin: AccountInfo<'info>,
}
