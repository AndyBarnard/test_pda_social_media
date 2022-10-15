use anchor_lang::prelude::*;
use std::mem::*;

use crate::state::Like;

pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    ctx.accounts.like.set_inner(
        Like::new(
            ctx.accounts.auth,
            *ctx.bumps.get("like").expect("couldnt get like bump")
        )
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(
        init,
        payer = auth,
        space = size_of::<Like>(),
        seeds = [b"like", auth.key().as_ref()],
        bump
    )]
    pub like: Account<'info, Like>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}