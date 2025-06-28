use solana_decode_error::DecodeError;
use solana_program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OperatorHistoryError {
    #[error("ArithmeticError")]
    ArithmeticError,
}

impl<T> DecodeError<T> for OperatorHistoryError {
    fn type_of() -> &'static str {
        "jito::operator_history"
    }
}

impl From<OperatorHistoryError> for ProgramError {
    fn from(value: OperatorHistoryError) -> Self {
        Self::Custom(value as u32)
    }
}

impl From<OperatorHistoryError> for u64 {
    fn from(e: OperatorHistoryError) -> Self {
        e as Self
    }
}

impl From<OperatorHistoryError> for u32 {
    fn from(e: OperatorHistoryError) -> Self {
        e as Self
    }
}
