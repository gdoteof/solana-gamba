use anchor_lang::{prelude::*};
use jet_proc_macros::assert_size;

#[assert_size(40)]
#[account(zero_copy)]
pub struct GambaAccount {
    pub authority: Pubkey,
    pub current_open_epoch: u32,
    pub latest_closed_epoch: u32
}


#[cfg(test)]
mod tests {
    use super::GambaAccount;

    #[test]
    fn bet_info_size() {
        println!("BetInfo: {}", std::mem::size_of::<GambaAccount>());
        static_assertions::const_assert_eq!(40, std::mem::size_of::<GambaAccount>());
    }

}