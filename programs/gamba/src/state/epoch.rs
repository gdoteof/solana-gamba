use anchor_lang::{prelude::*};


#[account(zero_copy)]
pub struct EpochAccount {
    pub epoch: u32,
    pub num_bets: u8,
    pub max_bets: u8,
    pub authority: Pubkey,
    pub bets: [u8; 1200]
}

