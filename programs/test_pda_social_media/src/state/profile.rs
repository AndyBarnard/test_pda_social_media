use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Profile {
    name: String,
    age: u64,
    likes: u64,
    bump: u8,
}

impl Profile {
    pub const SEED_PREFIX: &'static str = "profile";

    pub fn new(name: String, age: u64, bump: u8) -> Self {
        Profile {
            name,
            age,
            bump,
        }
    }
}