use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;

use crate::instructions::*;
use crate::state::*;
use crate::state::PrizePool;
use crate::state::Question;

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
    let question_id = ...; // Get question ID from context or instruction data.
    let reward_amount = ...; // Calculate reward based on logic.

    ctx.check_sufficient_funds(reward_amount)?;
    ctx.update_total_pool(-reward_amount)?;

    // ... Update question status, distribute rewards, etc.

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
