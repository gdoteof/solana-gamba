use anchor_lang::{prelude::*};

extern crate static_assertions;


pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod bet;

use errors::ErrorCode;
use instructions::*;

declare_id!("HaSj7pdndQD9DepFmPrcyL7exQ1BDUfr1qG4Uaxypfa9");


#[program]
pub mod gamba {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUserAccount>, bump: u8,  user_name: String, authority : Pubkey) -> ProgramResult {
        instructions::init_user::handler(ctx, bump, user_name, authority)
    }

    pub fn initialize_gamba(ctx: Context<InitializeGamba>, bump: u8, authority : Pubkey) -> ProgramResult {
        instructions::init_gamba::handler(ctx, bump, authority)
    }

    pub fn initialize_epoch(ctx: Context<InitializeEpoch>, bump: u8, epoch: u32) -> ProgramResult {
        instructions::init_epoch::handler(ctx, bump, epoch)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use solana_program::pubkey::Pubkey;


    #[test]
    fn dump_user_pdas() {

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
    #[test]
    fn dump_gamba_pdas() {

        let program_pubkey  :Pubkey = "HaSj7pdndQD9DepFmPrcyL7exQ1BDUfr1qG4Uaxypfa9".try_into().unwrap();
        let authority_pubkey: Pubkey = "Bw3PEQho6Svz2CuP7SD18ot91q2W4FYcLz6V4oUeLviS".try_into().unwrap();
        let (pda, _bump_seed) = Pubkey::find_program_address(&[&authority_pubkey.to_bytes(), b"gamba"], &program_pubkey);
        println!("pda is: {}\n bump is: {}", pda.to_string(), _bump_seed);
        println!("program id  is: {}\n authority is: {}", program_pubkey.to_string(), authority_pubkey.to_string());
        println!("program key bytes: {:X?}", program_pubkey.to_bytes());
        println!("authority key bytes: {:X?}", authority_pubkey.to_bytes());
        println!("gamba bytes {:?}", b"gamba");

//        "user account pda: 9hTJzJ53GsM7MLSCuuY6XiavHPUcSxu44uWJWAkdnHQG
//        authority publicid Bw3PEQho6Svz2CuP7SD18ot91q2W4FYcLz6V4oUeLviS"

// authority publicid Bw3PEQho6Svz2CuP7SD18ot91q2W4FYcLz6V4oUeLviS
// program id HaSj7pdndQD9DepFmPrcyL7exQ1BDUfr1qG4Uaxypfa9
// user account pda: 8CLRa6vwsBxLCv2PjhfDZ6cgkoFVCur4gzGq4t6aAbAt
// user account bump: 254


    }
}