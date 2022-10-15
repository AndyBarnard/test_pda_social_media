use anchor_lang::prelude::*;

#[account]
pub struct Post {
    header: String,
    body: String,
    bump: u8,
}

impl Post {
    pub const ACCOUNT_SPACE: usize = 8 + 32;
    pub const SEED_PREFIX: &'static str = "post";

    pub fn new(header: String, body: String, bump: u8) -> Self {
        Post {
            header,
            body,
            bump
        }
    }
}