use anchor_lang::{prelude::*};
use jet_proc_macros::assert_size;
use bytemuck::{Pod, Zeroable, Contiguous};

use crate::{utils::{FixedBuf, StoredPubkey}, errors::ErrorCode};



#[account(zero_copy)]
pub struct EpochAccount {
    pub epoch: u32,
    pub num_bets: u8,
    pub max_bets: u8,
    pub authority: Pubkey,
    pub bets: [u8; 1200]
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
    bet_info: [BetInfo; 32],
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

#[assert_size(4)]
#[derive(Contiguous, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum BetType {
    TwoFold,
    TenFold,
}

#[assert_size(4)]
#[derive(Contiguous, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum BetChoice {
    Low,
    High,
}

#[assert_size(aligns,128)]
#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct BetInfo {
    /// The related user account
    pub user: StoredPubkey, //32

    pub lamports: u32, //4

    pub bet_type: u32, //4

    pub bet_choice: u32, //4

    /// Unused space
    _reserved: FixedBuf<84>, //84

}

#[cfg(test)]
mod tests {
    use super::BetInfo;

    #[test]
    fn bet_info_size() {
        println!("BetInfo: {}", std::mem::size_of::<BetInfo>());
        static_assertions::const_assert_eq!(128, std::mem::size_of::<BetInfo>());
    }

}
