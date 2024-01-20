use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// API model for Wallets.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletsApi {
    pub user_id: i32,
    pub sol_addr: Pubkey,
}

/// API model for Responses.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponsesApi {
    pub response_id: i32,
    pub daredevil_id: i32,
    pub question_id: i32,
    pub response_code: String,
    pub correctness: bool,
    pub creation_time: NaiveDateTime,
}

/// API model for Questions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionsApi {
    pub question_id: i32,
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub deadline: NaiveDateTime,
    pub category: String,
}

/// API model for TestCases.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestCasesApi {
    pub test_case_id: i32,
    pub question_id: i32,
}

/// API model for Users.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersApi {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub total_payed: i32,
    pub total_claimed: i32,
}
