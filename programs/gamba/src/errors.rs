
use anchor_lang::{prelude::*};

#[error]
pub enum ErrorCode {
    #[msg("Bad Epoch, the epoch supplied is not correct.")]
    BadEpoch,
}
