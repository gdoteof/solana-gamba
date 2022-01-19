use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(gamba_bump: u8, user_bump: u8)]
pub struct InitializeUserAccount<'info> {
    #[
        account(init, 
        payer = authority,
        seeds = [authority.key.as_ref(), b"user_account".as_ref()], 
        bump = user_bump,
        space = 8 + 16 + 200
    )]
    pub user_account: Account<'info, UserAccount>,

    #[
        account(
        seeds = [b"gamba".as_ref()], 
        bump = gamba_bump,
    )]
    pub gamba_account: AccountLoader<'info, GambaAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitializeUserAccount>, _gamba_bump: u8,  _user_bump: u8, user_name: String, authority : Pubkey) -> ProgramResult {
    let user_account = &mut ctx.accounts.user_account;
    user_account.user_name = user_name;
    user_account.authority = authority;

    Ok(())
}