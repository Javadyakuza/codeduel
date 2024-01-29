use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

use crate::instructions::*;
use crate::state::PrizePool;
use crate::state::Question;
use crate::state::*;

pub mod instructions;
pub mod state;

impl Context<Context> {
    pub fn prize_pool(&self) -> &AccountInfo<'_, PrizePool> {
        &self.accounts.prize_pool
    }

    pub fn questions(&self) -> &AccountInfo<'_, Questions> {
        &self.accounts.questions
    }

    pub fn rivals(&self) -> &AccountInfo<'_, Rivals> {
        &self.accounts.rivals
    }

    pub fn rivals_balances(&self) -> &AccountInfo<'_, RivalsBalances> {
        &self.accounts.rivals_balances
    }
}

pub fn deposit(ctx: Context<Context>, sol: u64, question_id: u32, amount: u64) -> ProgramResult {
    instructions::deposit(ctx, sol, question_id, amount)
}

pub fn open_question(ctx: Context<Context>, question: Question) -> ProgramResult {
    instructions::open_question(ctx, question)
}

pub fn close_question(
    ctx: Context<Context>,
    status: QuestionStatus,
    question_id: u32,
) -> ProgramResult {
    instructions::close_question(ctx, status, question_id)
}

pub fn update_total_pool(&self, amount: u64) -> ProgramResult {
    let mut prize_pool_data = self.prize_pool().try_borrow_mut_data()?;
    prize_pool_data.total_pool += amount;
    Ok(())
}

pub fn check_sufficient_funds(&self, amount: u64) -> Result<(), ProgramError> {
    if self.prize_pool().total_pool < amount {
        return Err(ErrorCode::InsufficientPrizePool.into());
    }
    Ok(())
}

pub fn claim_reward(ctx: Context<Context>, daredevil_id: u32) -> ProgramResult {
    let question_id = ctx.accounts.questions.key(); // Get question ID from context

    // Get the question data
    let question = ctx.questions().load()?;

    // Ensure the question is closed and solved
    if question.status != QuestionStatus::ClosedSolved {
        return Err(ErrorCode::InvalidQuestionStatus.into());
    }

    // Calculate reward amount
    let reward_amount = question.reward + question.entrance_fee;

    // Check for sufficient funds in the prize pool
    ctx.check_sufficient_funds(reward_amount)?;

    // Update the prize pool
    ctx.update_total_pool(-reward_amount)?;

    // Mark the question as rewarded
    question.rewarded = true;
    ctx.questions().save(&question)?;

    // Reward the daredevil using the `reward_daredevil` helper function
    reward_daredevil(ctx, daredevil_id, reward_amount)?;

    // Refund rivals (if applicable) using the `refund_rivals` helper function
    refund_rivals(ctx, question_id, question.reward + question.entrance_fee)?;

    Ok(())
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> ProgramError {
        ProgramError::Custom(e.code as u32)
    }
}

pub enum ErrorCode {
    InvalidAmount,
    InsufficientEntranceFee,
    InsufficientPrizePool,
}
