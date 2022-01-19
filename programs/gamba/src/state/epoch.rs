use anchor_lang::{prelude::*};
use jet_proc_macros::assert_size;
use bytemuck::{Pod, Zeroable, Contiguous};

use crate::{errors::ErrorCode};

use super::{BetInfo, BetType, BetChoice};

pub const MAX_BETS: i8 = 32;

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
    bet_info: [BetInfo; MAX_BETS as usize],
}

pub type BetIndex = u16;


impl EpochBets {
    pub fn register(&mut self, user: &Pubkey, lamports : u32, bet_type: BetType, bet_choice: BetChoice) -> Result<BetIndex, ErrorCode> {
        for (index, entry) in self.bet_info.iter_mut().enumerate() {
            if entry.user != Pubkey::default() {
                continue;
            }
            *entry.user = *user;
            entry.lamports = lamports;
            entry.bet_type = bet_type.into_integer();
            entry.bet_choice = bet_choice.into_integer();

            return Ok(index as BetIndex);
        }
        Err(ErrorCode::EpochFull)
    }

    pub fn remove(&mut self, index: BetIndex) {
        self.bet_info[index as usize] = BetInfo::zeroed();
    }

    pub fn get_mut(&mut self, index: BetIndex) -> &mut BetInfo {
        &mut self.bet_info[index as usize]
    }

    pub fn get(&self, index: BetIndex) -> &BetInfo {
        &self.bet_info[index as usize]
    }
}


