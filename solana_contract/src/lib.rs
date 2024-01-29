use anchor_lang::prelude::*;
pub mod instructions;
pub mod processor;
pub mod state;
pub mod error;

pub fn validate_amount(amount: u64) -> Result<(), ProgramError> {
    if amount == 0 {
        return Err(ErrorCode::InvalidAmount.into());
    }
    Ok(())
}

pub fn calculate_entrance_fee(reward: u64, code_duel_fee: u64) -> u64 {
    (reward * code_duel_fee) / 100
}

pub fn distribute_prize(
    prize_pool: AccountInfo<'_, PrizePool>,
    winning_daredevil: u32,
    losing_rivals: &[u32],
    rivals_balances: AccountInfo<'_, RivalsBalances>,
) -> ProgramResult {
    let prize_pool_balance = prize_pool.try_borrow_data()?.total_pool;
    let mut rivals_balances_data = rivals_balances.try_borrow_mut_data()?;

    let daredevil_share = prize_pool_balance / 2;
    rivals_balances_data.balances.insert(winning_daredevil, daredevil_share);

    let rival_share = prize_pool_balance / (losing_rivals.len() as u64 + 1);
    for rival in losing_rivals {
        rivals_balances_data.balances.insert(*rival, rival_share);
    }

    let remaining_pool = prize_pool_balance - daredevil_share - (rival_share * losing_rivals.len() as u64);
    prize_pool.try_borrow_mut_data()?.total_pool = remaining_pool;

    Ok(())
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> ProgramError {
        ProgramError::Custom(e as u32)
    }
}

pub enum ErrorCode {
    InvalidAmount,
}

// Calculate a deadline by adding a specified duration to the current timestamp.
pub fn calculate_deadline(duration: Duration) -> Timestamp {
    Timestamp::now() + duration
}

// Check if a timestamp has passed.
pub fn has_deadline_passed(deadline: Timestamp) -> bool {
    Timestamp::now() >= deadline
}

// Generate a random seed for cryptographic operations.
pub fn generate_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    // Use a cryptographically secure random number generator to populate the seed.
    rand::thread_rng().fill(&mut seed);
    seed
}

// Hash a string using SHA-256.
pub fn hash_string(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string);
    let hash = hasher.finalize();
    hex::encode(hash)
}

// Validate a question ID, ensuring it's within a valid range.
pub fn validate_question_id(id: u32) -> Result<(), ProgramError> {
    if id < 1 || id > 1000 { // Adjust the range as needed.
        return Err(ErrorCode::InvalidQuestionId.into());
    }
    Ok(())
}

// Validate a user's input, ensuring it meets certain criteria.
pub fn validate_user_input(input: &str) -> Result<(), ProgramError> {
    // Perform checks on the input, such as length, allowed characters, etc.
    if !input.is_ascii() || input.len() > 50 { // Adjust rules as needed.
        return Err(ErrorCode::InvalidInput.into());
    }
    Ok(())
}

// Fetch a price from a Chainlink oracle.
use solana_client::rpc_client::RpcClient;
use chainlink_solana as chainlink;

pub async fn fetch_price(client: &RpcClient, oracle_program_id: &Pubkey, price_feed: &Pubkey) -> Result<u64, ProgramError> {
    let price = chainlink::get_price(client, oracle_program_id, price_feed).await?;
    Ok(price)
}