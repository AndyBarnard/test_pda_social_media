use anchor_lang::prelude::*;

pub fn create_cpi_state(ctx: Context<CreateCpiState>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateCpiState<'info> {
    #[account(init, payer = auth, space = 8 + 8)]
    pub cpi_state: Account<'info, CpiState>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub system_program: Program<'info, System>
}