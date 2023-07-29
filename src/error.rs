use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    // Error 0
    #[error("PDA derived does not equal provided PDA")]
    InvalidPDA,

    // Error 1
    #[error("Ship is not the active ship")]
    InvalidShip,

    // Error 2
    #[error("Ship is not a valid target ship")]
    InvalidTarget,
}

impl From<GameError> for ProgramError {
    fn from(e: GameError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
