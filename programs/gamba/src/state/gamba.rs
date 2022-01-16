use anchor_lang::{prelude::*};

#[account]
pub struct GambaAccount {
    pub authority: Pubkey,
    pub current_open_epoch: u32,
    pub latest_closed_epoch: u32
}