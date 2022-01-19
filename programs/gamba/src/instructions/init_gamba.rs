use anchor_lang::{prelude::*};
use crate::state::*;

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct InitializeGamba<'info> {
    #[
        account(init, 
        payer = authority,
        seeds = [b"gamba".as_ref()], 
        bump = _bump,
        space = 8 + 40
    )]
    pub gamba_account: AccountLoader<'info, GambaAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitializeGamba>, _bump: u8, authority : Pubkey) -> ProgramResult {
    let mut gamba_account = ctx.accounts.gamba_account.load_init()?;
    gamba_account.current_open_epoch = 1;
    gamba_account.latest_closed_epoch = 0;
    gamba_account.authority = authority;
    Ok(())
}