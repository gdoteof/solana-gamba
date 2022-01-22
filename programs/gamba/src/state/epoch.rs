use anchor_lang::{prelude::*};
use jet_proc_macros::assert_size;
use bytemuck::{Pod, Zeroable};

use crate::{errors::ErrorCode, utils::StoredPubkey};

pub const MAX_BETS: u8 = 128;

#[assert_size(4140)]
#[account(zero_copy)]
pub struct EpochAccount {
    pub epoch: u32,
    pub num_bets: u32,
    pub max_bets: u32,
    pub authority: Pubkey,
    pub bets: [u8; 4096]
}

impl EpochAccount {
    pub fn bets_mut(&mut self) -> &mut EpochBets {
            bytemuck::from_bytes_mut(&mut self.bets)
    }

    pub fn bets(&self) -> &EpochBets {
            bytemuck::from_bytes(&self.bets)
    }
}

#[assert_size(aligns, 4096)]
#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct EpochBets {
    bet_info: [StoredPubkey; MAX_BETS as usize],
}

pub type BetIndex = u16;


impl EpochBets {
    pub fn register(&mut self, bet_account: &Pubkey) -> Result<BetIndex, ErrorCode> {
        for (index, entry) in self.bet_info.iter_mut().enumerate() {
            if *entry != Pubkey::default() {
                continue;
            }
            *entry = (*bet_account).into();

            return Ok(index as BetIndex);
        }
        Err(ErrorCode::EpochFull)
    }

    pub fn remove(&mut self, index: BetIndex) {
        self.bet_info[index as usize] = Pubkey::default().into()
    }

    pub fn get_mut(&mut self, index: BetIndex) -> &mut StoredPubkey {
        &mut self.bet_info[index as usize]
    }

    pub fn get(&self, index: BetIndex) -> &StoredPubkey {
        &self.bet_info[index as usize]
    }
}


