use anchor_lang::{prelude::*};
use crate::state::*;

#[account]
pub struct EpochAccount {
    pub epoch: u32,
    pub bets: Vec<Bet>,
    pub num_bets: u8,
    pub max_bets: u8,
    pub authority: Pubkey
}