use anchor_lang::{prelude::*};
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gamba {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _user_pubkey: Pubkey, _user_account_bump: u8, user_name: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        user_account.user_name = user_name;
        Ok(())
    }

    pub fn make_bet(_ctx: Context<MakeBet>, _bet_type: BetType, _bet_choice: BetChoice, _amount: u32) -> ProgramResult {
        Ok(())
    }
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

#[derive(Accounts)]
#[instruction(_user_pubkey: Pubkey)]
pub struct Initialize<'info> {
    #[
        account(init, 
        payer = authority,
        has_one = authority,
        seeds = [_user_pubkey.as_ref(), b"user_account"], 
        bump,
        space = 8 + 16
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>

}

#[derive(Accounts)]
pub struct MakeBet<'info> {
    #[account(init, payer = user, space = 1 + 4)]
    pub bet_account: Account<'info, BetAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>

}

#[account]
pub struct BetAccount {
    pub bet_type: BetType,
    pub bet_choice: BetChoice,
    pub amount: u32
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub user_name: String
}