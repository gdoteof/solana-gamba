use anchor_lang::{prelude::*};
use crate::state::*;

#[derive(Accounts)]
pub struct MakeBet<'info> {
    #[account(init, payer = user, space = 1 + 4)]
    pub bet_account: AccountLoader<'info, BetAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>

}