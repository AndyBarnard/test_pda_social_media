use anchor_lang::prelude::*;
use std::mem::*;

use crate::state::Post;

pub fn create_post(ctx: Context<CreatePost>, header: String, body: String) -> Result<()> {
    ctx.accounts.post.set_inner(
        Post::new(
            header,
            body,
            *ctx.bumps.get("post").expect("Post bump not found")
        )
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        init,
        payer = auth,
        space = size_of::<Post>(),
        seeds = [b"post", auth.key().as_ref()],
        bump
    )]
    pub post : Account<'info, Post>,
    #[account(mut)]
    pub auth: Signer<'info>,
    pub system_program: Program<'info, System>,
}