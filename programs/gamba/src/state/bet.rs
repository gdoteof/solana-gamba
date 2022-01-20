use anchor_lang::{prelude::*};


#[account]
pub struct BetAccount {
    pub user: Pubkey,
    pub bet_type: BetType,
    pub bet_choice: BetChoice,
    pub lamports: u32
}

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


#[cfg(test)]
mod tests {

    #[test]
    fn bet_info_size() {
    }

}