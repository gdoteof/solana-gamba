use anchor_lang::{prelude::*};
use bytemuck::{Pod, Zeroable, Contiguous};
use jet_proc_macros::assert_size;

use crate::{utils::{FixedBuf, StoredPubkey}};


#[account(zero_copy)]
pub struct BetAccount {
    pub user: Pubkey,
    pub bet_type: u32,
    pub bet_choice: u32,
    pub lamports: u32
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