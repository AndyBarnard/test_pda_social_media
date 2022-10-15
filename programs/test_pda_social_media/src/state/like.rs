use anchor_lang::prelude::*;

#[account]
pub struct Like {
    pub liker: Pubkey,
    pub bump: u8
}

impl Like {
    pub fn new(liker: Pubkey, bump: u8) -> Self {
        Like {
            liker,
            bump
        }
    }
}