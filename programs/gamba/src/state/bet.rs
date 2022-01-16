use anchor_lang::{prelude::*};
use borsh::{BorshDeserialize, BorshSerialize};

#[account]
pub struct BetAccount {
    pub bet_type: BetType,
    pub bet_choice: BetChoice,
    pub amount: u32
}

#[derive(PartialEq, Eq, BorshDeserialize, BorshSerialize, Clone)]
pub enum BetType {
    Double,
    TenFold, 
}

#[derive(PartialEq, Eq, BorshSerialize, BorshDeserialize, Clone)]
pub enum BetChoice {
    Low,
    High,
}

#[derive(PartialEq, Eq, BorshSerialize, BorshDeserialize, Clone)]
pub struct Bet {
    bet_type: BetType,
    bet_choice: BetChoice,
    amount: u32,
    user: Pubkey
}