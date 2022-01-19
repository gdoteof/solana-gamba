use anchor_lang::{prelude::*};
use bytemuck::Contiguous;
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

pub fn handler(ctx: Context<MakeBet>, _bump: u8, epoch: u32, user: Pubkey, lamports: u32, 
bet_type: BetType, bet_choice: BetChoice) -> ProgramResult {
    let gamba_account = ctx.accounts.gamba_account.load()?;
    let mut bet_account = ctx.accounts.bet_account.load_init()?;
    let mut epoch_account = ctx.accounts.epoch_account.load_mut()?;

    if gamba_account.current_open_epoch + 1 != epoch {
        return Err(ErrorCode::BadEpoch.into());
    }

    bet_account.user = user;
    bet_account.lamports = lamports;
    bet_account.bet_type = bet_type.into_integer();
    bet_account.bet_choice = bet_choice.into_integer();

    epoch_account.bets_mut().register(&user, lamports, bet_type, bet_choice)?;

    Ok(())
}