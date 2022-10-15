use anchor_lang::prelude::*;

use crate::state::Profile;

pub fn change_profile_name(ctx: Context<ChangeProfileName>, new_name: String) -> Result<()> {
    ctx.accounts.profile.name = new_name;

    Ok(())
}

#[derive(Accounts)]
pub struct ChangeProfileName<'info> {
    #[account(
        mut, 
        seeds = [b"profile", auth.key().as_ref()], 
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,
    pub auth: Signer<'info>,
}