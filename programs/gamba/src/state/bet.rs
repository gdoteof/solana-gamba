use anchor_lang::{prelude::*};
use jet_proc_macros::assert_size;


#[derive(Debug, Clone, Copy, Eq, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum BetType {
    TwoFold,
    TenFold,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum BetChoice {
    Low,
    High,
}

#[assert_size(40)]
#[account]
pub struct BetAccount {
    pub user: Pubkey,
    pub bet_type: BetType,
    pub bet_choice: BetChoice,
    pub lamports: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bet_account_size() {
        println!("BetAccount: {}", std::mem::size_of::<BetAccount>());
    }

}