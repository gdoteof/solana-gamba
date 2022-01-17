
use anchor_lang::{prelude::*};

#[error]
pub enum ErrorCode {
    #[msg("Bad Epoch, the epoch supplied is not correct.")]
    BadEpoch,
    #[msg("All the bets have been filled for this epoch")]
    EpochFull,
}
