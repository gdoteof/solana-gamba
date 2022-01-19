use anchor_lang::prelude::*;
use crate::state::*;
use crate::ErrorCode;


#[derive(Accounts)]
#[instruction(epoch: u32, epoch_bump: u8, gamba_bump: u8)]
pub struct InitializeEpoch<'info> {
    #[
        account(init, 
        payer = authority,
        seeds = [&epoch.to_le_bytes(), b"epoch_account".as_ref()], 
        bump = epoch_bump,
        space = 8 + 4140,
    )]
    pub epoch_account: AccountLoader<'info, EpochAccount>,

    #[account(mut, 
        seeds = [b"gamba".as_ref()], 
        bump = gamba_bump,
        has_one=authority)]
    pub gamba_account: AccountLoader<'info, GambaAccount>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitializeEpoch>, epoch: u32, _epoch_bump: u8, _gamba_bump: u8) -> ProgramResult {
    let mut gamba_account = ctx.accounts.gamba_account.load_mut()?;
    let mut epoch_account = ctx.accounts.epoch_account.load_init()?;

    if gamba_account.current_open_epoch + 1 != epoch {
        return Err(ErrorCode::BadEpoch.into());
    }

    gamba_account.current_open_epoch = epoch;
    epoch_account.epoch = epoch;
    epoch_account.authority = gamba_account.authority;
    epoch_account.num_bets = 0;
    epoch_account.max_bets = MAX_BETS as u32;

    Ok(())
}