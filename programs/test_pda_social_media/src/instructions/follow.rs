use anchor_lang::prelude::*;

use crate::state::Profile;

pub fn follow(ctx: Context<Follow>) -> Result<()> {
    ctx.accounts.profile.likes += 1;
}

#[derive(Accounts)]
pub struct Follow<'info> {
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub system_program: Program<'info, System>
}