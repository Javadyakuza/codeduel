use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

mod instructions;
mod state;

#[program]
pub mod code_duel {
    use super::*;

    pub fn process_instruction(ctx: Context<Context>, instruction: Instruction) -> ProgramResult {
        match instruction {
            Instruction::Deposit(deposit_ctx) => instructions::deposit(ctx, deposit_ctx),
            Instruction::OpenQuestion(open_question_ctx) => {
                instructions::open_question(ctx, open_question_ctx)
            }
            Instruction::CloseQuestion(close_question_ctx) => {
                instructions::close_question(ctx, close_question_ctx)
            }
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum Instruction {
    Deposit(instructions::DepositContext),
    OpenQuestion(instructions::OpenQuestionContext),
    CloseQuestion(instructions::CloseQuestionContext),
}

pub mod accounts {
    use super::state::*;
    use anchor_lang::prelude::*;

    #[account]
    pub struct PrizePool {
        pub total_pool: u64,
    }

    #[account]
    pub struct Questions {
        pub entries: HashMap<u32, Question>,
    }

    #[account]
    pub struct Rivals {
        pub rival_questions: HashMap<u32, Vec<u32>>,
    }

    #[account]
    pub struct RivalsBalances {
        pub balances: HashMap<u32, u64>,
    }

    #[derive(AnchorDeserialize, AnchorSerialize)]
    pub struct Question {
        pub id: u32,
        pub name: String,
        pub status: QuestionStatus,
        pub reward: u64,
        pub rival_id: u32,
        pub daredevil_id: u32,
        pub current_prize_pool: u64,
        pub entrance_fee: u64,
        pub deadline: Timestamp,
    }

    pub enum QuestionStatus {
        Open,
        OpenNotSolved,
        ClosedSolved,
        ClosedNotSolved,
    }
}

pub use state::*;

impl From<AnchorError> for ProgramError {
    fn from(e: AnchorError) -> ProgramError {
        match e {
            AnchorError::AccountBorrowFailed => ProgramError::AccountBorrowFailed,
            AnchorError::AccountDeserializeMismatch => ProgramError::AccountDeserializeMismatch,
            AnchorError::AccountNotFound => ProgramError::AccountNotFound,
            AnchorError::ConstraintDidNotResolve => ProgramError::ConstraintDidNotResolve,
            AnchorError::InstructionDataTooLarge => ProgramError::InstructionDataTooLarge,
            AnchorError::InvalidAccountOwner => ProgramError::InvalidAccountOwner,
            AnchorError::ProgramError(err) => err,
        }
    }
}
