use diesel::prelude::*;
// use merge_derivable;
use crate::schema::{questions, responses, test_cases, users};
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};

// general models
#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wallets {
    pub user_id: i32,
    pub sol_addr: String,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::responses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Responses {
    pub response_id: i32,
    pub daredevil_id: i32,
    pub question_id: i32,
    pub response_code: String,
    pub correctness: bool,
    pub creation_time: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Questions {
    pub question_id: i32,
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub creation_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub category: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::test_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TestCases {
    pub test_case_id: i32,
    pub question_id: i32,
    pub test_inputs: String,
    pub test_outputs: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub total_payed: i32,
    pub total_claimed: i32,
}

// inserting models
// the following models are not containing the primary key fields, making able the fns to insert the values without the pks.

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::responses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IResponses {
    // pub response_id: i32, // pk
    pub daredevil_id: i32,
    pub question_id: i32,
    pub response_code: String,
    pub correctness: bool,
    pub creation_time: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IQuestions {
    // pub question_id: i32, // pk
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub creation_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub category: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::test_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ITestCases {
    // pub test_case_id: i32, // pk
    pub question_id: i32,
    pub test_inputs: String,
    pub test_outputs: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IUsers {
    // pub user_id: i32, // pk
    pub email: String,
    pub username: String,
    pub password: String,
    pub total_payed: i32,
    pub total_claimed: i32,
}