use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/*
social media app to test my understanding of PDAs

User
Profile
Post

one User maps to one Profile
one account maps to many Posts
*/

/*
PDA bumps

you use the one giant macro in the Transaction struct to make a new account w/ bump
you get the bump from *ctx in the handler fn
the struct you want indexed by the bump should have a bump field
when you want to access the bump, the struct you pass for the specific tx where you
    access will have e.g. the following macro:

    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserStats>,

so to access the bump, you're using the macro to pull it from the field on the struct of the
tx accessing it which is type :: Account<'info, T>

*****
PDAs
*****

*/

#[program]
pub mod test_pda_social_media {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
        instructions::create_profile::create_profile(ctx, String::from("name"), 69)
    }

    pub fn create_post(ctx: Context<CreatePost>) -> Result<()> {
        instructions::create_post::create_post(ctx, String::from("header"), String::from("body"))
    }

    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::create_like::create_like(ctx)
    }

    pub fn change_profile_name(ctx: Context<ChangeProfileName>) -> Result<()> {
        instructions::change_profile_name::change_profile_name(ctx, String::from("new_name"))
    }

    pub fn follow(ctx: Context<Follow>) -> Result<()> {
        instructions::follow::follow(ctx)
    }
}