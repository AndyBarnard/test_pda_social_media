use anchor_lang::prelude::*;

#[account]
pub struct CpiState {
    pub data: u64
}