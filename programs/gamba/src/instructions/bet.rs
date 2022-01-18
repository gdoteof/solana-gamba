use anchor_lang::{prelude::*};
use crate::{state::*, errors::ErrorCode};

#[derive(Accounts)]
#[instruction(bet_bump: u8, gamba_bump: u8, epoch_bump: u8, epoch: u32)]
pub struct MakeBet<'info> {
    #[account(init, 
        seeds = [user.key.as_ref(), b"bet".as_ref()], 
        bump = bet_bump,
        payer = user, space = 8 + 12)]
    pub bet_account: AccountLoader<'info, BetAccount>,

    #[
        account(
        seeds = [b"gamba".as_ref()], 
        bump = gamba_bump,
    )]
    pub gamba_account: AccountLoader<'info, GambaAccount>,

    #[
        account(
        seeds = [&epoch.to_le_bytes(), b"epoch_account".as_ref()], 
        mut,
        bump = epoch_bump,
    )]
    pub epoch_account: AccountLoader<'info, EpochAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>

}

pub fn handler(ctx: Context<MakeBet>, bump: u8, epoch: u32, user: Pubkey, lamports: u32, 
bet_type: BetType, bet_choice: BetChoice) -> ProgramResult {
    let gamba_account = ctx.accounts.gamba_account.load()?;
    let bet_account = ctx.accounts.bet_account.load_init()?;
    let epoch_account = ctx.accounts.epoch_account.load()?;

    if gamba_account.current_open_epoch + 1 != epoch {
        return Err(ErrorCode::BadEpoch.into());
    }

    bet_account.user = user;


    Ok(())
}