use anchor_lang::prelude::*;

#[derive(ErrorCode)]
pub enum ErrorCode {
    // Deposit instruction errors
    InvalidAmount,
    InsufficientEntranceFee,

    // OpenQuestion instruction errors
    DuplicateQuestion, // Question with same ID already exists

    // CloseQuestion instruction errors
    UnknownQuestion,       // Question ID not found
    InvalidQuestionStatus, // Invalid status for closing a question

    // General errors
    AccountBorrowFailed,
    AccountDeserializeMismatch,
    AccountNotFound,
    ConstraintDidNotResolve,
    InstructionDataTooLarge,
    InvalidAccountOwner,
    ProgramError(ProgramError), // Handle generic Solana program errors
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> ProgramError {
        ProgramError::Custom(e as u32)
    }
}
