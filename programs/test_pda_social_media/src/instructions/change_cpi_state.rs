use anchor_lang::prelude::*;

pub fn change_cpi_state(ctx: Context<CreateCpiState>, data: u64) -> Result<()> {
    ctx.accounts.cpi_state.data = data;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ChangeCpiState<'info> {
    #[account(mut)]
    pub cpi_state: Account<'info, CpiState>
}