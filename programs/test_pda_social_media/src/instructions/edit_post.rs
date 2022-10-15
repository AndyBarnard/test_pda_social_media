use anchor_lang::prelude::*;

use crate::state::*;

pub fn edit_post(ctx: Context<EditPost>, new_header: Option<String>, new_body: Option<String>) -> Result<()> {
    if let Some(new_header) = new_header {
        ctx.accounts.post.header = new_header;
    }
    if let Some(new_body) = new_body {
        ctx.accounts.post.body = new_body;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct EditPost<'info> {
    #[account(
        mut, 
        seeds = [b"post", auth.key().as_ref()], 
        bump = post.bump
    )]
    pub post: Account<'info, Post>,
    pub auth: Signer<'info>,
}