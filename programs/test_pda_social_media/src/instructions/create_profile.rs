use anchor_lang::prelude::*;

use crate::state::Profile;

pub fn create_profile(ctx: Context<CreateProfile>, name: String, age: u64) -> Result<()> {
    ctx.accounts.profile.set_inner(
        Profile::new(
            name,
            age,
            *ctx.bumps.get("profile").expect("Bump not found"),
        )
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = auth,
        space = 8 + 2 + 4 + 200 + 1, 
        seeds = [b"profile", auth.key().as_ref()], 
        bump
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub system_program: Program<'info, System>
}