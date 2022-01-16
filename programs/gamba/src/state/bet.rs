use anchor_lang::{prelude::*};

#[account(zero_copy)]
pub struct BetAccount {
    pub bet_type: u8,
    pub bet_choice: u8,
    pub amount: u32
}
