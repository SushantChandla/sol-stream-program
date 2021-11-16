use thiserror::Error;

use solana_program::{msg, program_error::ProgramError};

#[derive(Error, Debug, Copy, Clone)]
pub enum StreamError {
    #[error("Failed to parse the pubkey")]
    PubKeyParseError,
    #[error("Admin account invalid")]
    AdminAccountInvalid,
    #[error("Not enough lamports in account")]
    NotEnoughLamports,
    #[error("Start time or end time for the stream is invalid")]
    InvalidStartOrEndTime,
    #[error("Receiver does not own enough tokens")]
    WithdrawError,
}

impl From<StreamError> for ProgramError {
    fn from(e: StreamError) -> Self {
        msg!("{}", e);
        ProgramError::Custom(e as u32)
    }
}
