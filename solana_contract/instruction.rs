#![allow(dead_code)]

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    instruction::Instruction as SolanaInstruction, program_error::ProgramError, pubkey::Pubkey,
};

/// Instructions supported by the Fee-Relayer program.
#[repr(C)]
#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum Instruction {
    Deposit {
        sol: u64,
        question_id: u32,
        amount: u64,
    },
    OpenQuestion {
        question: Question,
    },
    CloseQuestion {
        status: QuestionStatus,
        question_id: u32,
    },
}

pub fn deposit(
    ctx: Context<DepositContext>,
    sol: u64,
    question_id: u32,
    amount: u64,
) -> ProgramResult {
    let accounts = ctx.accounts;
    let mut prize_pool = accounts.prize_pool.try_borrow_mut()?;
    let entrance_fee = questions_pool.get(&question_id)?.entrance_fee;

    require!(sol >= amount, ErrorCode::InvalidAmount);

    if amount < entrance_fee {
        return Err(ErrorCode::InsufficientEntranceFee);
    }

    let deposit_amount = amount - entrance_fee;

    prize_pool.insert(
        question_id,
        prize_pool.get(&question_id).unwrap() + deposit_amount,
    );

    Ok(())
}

pub fn open_question(ctx: Context<OpenQuestionContext>, question: Question) -> ProgramResult {
    let accounts = ctx.accounts;
    let mut questions = accounts.questions.try_borrow_mut()?;
    let mut rivals = accounts.rivals.try_borrow_mut()?;
    let prize_pool = accounts.prize_pool.try_borrow()?;

    require!(
        prize_pool.get(&question.id).unwrap() >= question.reward,
        ErrorCode::InsufficientPrizePool
    );

    questions.insert(question.id, question);

    let mut rival_ids = Vec::new();
    for dared_evil in question.dared_evils {
        rivals.insert(dared_evil, rival_ids.clone());
        rival_ids.push(question.id);
    }

    Ok(())
}

pub fn close_question(
    ctx: Context<CloseQuestionContext>,
    status: QuestionStatus,
    question_id: u32,
) -> ProgramResult {
    let accounts = ctx.accounts;
    let mut questions = accounts.questions.try_borrow_mut()?;
    let mut rivals = accounts.rivals.try_borrow_mut()?;
    let mut rivals_balances = accounts.rivals_balances.try_borrow_mut()?;
    let prize_pool = accounts.prize_pool.try_borrow_mut()?;

    let question = questions.get(&question_id).unwrap();

    if status == QuestionStatus::ClosedSolved {
        rivals_balances.insert(
            question.daredevil_id,
            rivals_balances.get(&question.daredevil_id).unwrap()
                + question.reward
                + question.entrance_fee,
        );
        prize_pool.insert(question_id, 0);
    } else {
        rivals_balances.insert(
            question.rival_id,
            rivals_balances.get(&question.rival_id).unwrap()
                + prize_pool.get(&question_id).unwrap(),
        );
        prize_pool.insert(question_id, 0);
    }

    questions.remove(&question_id);

    for dared_evil in question.dared_evils {
        let mut dared_evil_questions = rivals.get_mut(&dared_evil).unwrap();
        dared_evil_questions.retain(|x| *x != question_id);
    }

    Ok(())
}

fn reward_daredevil(ctx: &Context<CloseQuestionContext>, receiver: Pubkey) -> ProgramResult {
    let accounts = ctx.accounts;
    let mut user_account = accounts.daredevil.try_borrow_mut()?;
    transfer(
        accounts.program_id,
        accounts.prize_pool,
        &user_account,
        question.reward + question.entrance_fee,
    )
}

fn refund_rival(
    ctx: &Context<CloseQuestionContext>,
    receiver: Pubkey,
    amount: u64,
) -> ProgramResult {
    let accounts = ctx.accounts;
    let mut user_account = accounts.rival.try_borrow_mut()?;
    transfer(
        accounts.program_id,
        accounts.prize_pool,
        &user_account,
        amount,
    )
}
