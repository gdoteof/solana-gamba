use anchor_lang::{prelude::*};

use crate::{BetType, BetChoice};



#[account]
pub struct BetAccount {
    pub user: Pubkey,
    pub bet_type: BetType,
    pub bet_choice: BetChoice,
    pub lamports: u32
}

#[cfg(test)]
mod tests {

    #[test]
    fn bet_info_size() {
    }

}