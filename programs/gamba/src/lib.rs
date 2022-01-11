use anchor_lang::{prelude::*};
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("HaSj7pdndQD9DepFmPrcyL7exQ1BDUfr1qG4Uaxypfa9");

#[program]
pub mod gamba {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _bump: u8,  user_name: String) -> ProgramResult {
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
#[instruction(_bump: u8)]
pub struct Initialize<'info> {
    #[
        account(init, 
        payer = authority,
        seeds = [authority.key.as_ref(), b"user_account".as_ref()], 
        bump = _bump,
        space = 8 + 16 + 200
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



#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use solana_program::pubkey::Pubkey;


    use super::*;
    #[test]
    fn rng_pack_unpack() {

        let program_pubkey  :Pubkey = "HaSj7pdndQD9DepFmPrcyL7exQ1BDUfr1qG4Uaxypfa9".try_into().unwrap();
        let authority_pubkey: Pubkey = "Bw3PEQho6Svz2CuP7SD18ot91q2W4FYcLz6V4oUeLviS".try_into().unwrap();
        let (pda, _bump_seed) = Pubkey::find_program_address(&[&authority_pubkey.to_bytes(), b"user_account"], &program_pubkey);
        println!("pda is: {}\n bump is: {}", pda.to_string(), _bump_seed);
        println!("program id  is: {}\n authority is: {}", program_pubkey.to_string(), authority_pubkey.to_string());
        println!("program key bytes: {:X?}", program_pubkey.to_bytes());
        println!("authority key bytes: {:X?}", authority_pubkey.to_bytes());
        println!("user_account bytes {:?}", b"user_account");

//        "user account pda: 9hTJzJ53GsM7MLSCuuY6XiavHPUcSxu44uWJWAkdnHQG
//        authority publicid Bw3PEQho6Svz2CuP7SD18ot91q2W4FYcLz6V4oUeLviS"

// authority publicid Bw3PEQho6Svz2CuP7SD18ot91q2W4FYcLz6V4oUeLviS
// program id HaSj7pdndQD9DepFmPrcyL7exQ1BDUfr1qG4Uaxypfa9
// user account pda: 8CLRa6vwsBxLCv2PjhfDZ6cgkoFVCur4gzGq4t6aAbAt
// user account bump: 254


    }
}