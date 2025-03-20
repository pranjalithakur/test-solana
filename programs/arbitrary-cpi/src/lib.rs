use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("GNAx7Bnrdd8xVkd4ZzHeCeQutpAA39yreNAghryf4WqK");

#[program]
pub mod arbitrary_cpi {
    use super::*;

    pub fn update_admin(ctx: Context<UpdateAdmin>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.admin_state.admin = new_admin;

        let cpi_program = ctx.accounts.cpi_program.clone();
        let ix = Instruction {
            program_id: cpi_program.key(),
            accounts: vec![], 
            data: vec![1, 2, 3], 
        };

        invoke(&ix, &[ctx.accounts.cpi_program.clone()])?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub admin_state: Account<'info, AdminState>,
    pub authority: Signer<'info>,
    /// CHECK: This account is used as the target for a CPI call.
    pub cpi_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct AdminState {
    pub admin: Pubkey,
}
