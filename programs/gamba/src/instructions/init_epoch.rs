use anchor_lang::prelude::*;
use crate::state::*;
use crate::ErrorCode;

#[derive(Accounts)]
#[instruction(epoch: u32, bump: u8)]
pub struct InitializeEpoch<'info> {
    #[
        account(init, 
        payer = authority,
        seeds = [&epoch.to_le_bytes(), b"epoch_account".as_ref()], 
        bump = bump,
        space = 8 + 16 + 200,
    )]
    pub epoch_account: AccountLoader<'info, EpochAccount>,

    #[account(mut, has_one=authority)]
    pub gamba_account: AccountLoader<'info, GambaAccount>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitializeEpoch>, _bump: u8, epoch: u32) -> ProgramResult {
    let mut gamba_account = ctx.accounts.gamba_account.load_init()?;
    let mut epoch_account = ctx.accounts.epoch_account.load_init()?;

    if gamba_account.current_open_epoch + 1 != epoch {
        return Err(ErrorCode::BadEpoch.into());
    }

    gamba_account.current_open_epoch = epoch;
    epoch_account.epoch = epoch;
    epoch_account.authority = gamba_account.authority;
    epoch_account.num_bets = 0;
    epoch_account.max_bets = 8;

    Ok(())
}