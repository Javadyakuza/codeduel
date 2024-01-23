use diesel::prelude::*;
// use merge_derivable;
use crate::schema::{questions, responses, test_cases, users};
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};

// ------------------------------- general models ----------------------------
// the following models will represent the actual schema of the tables in terms of the rust structs.
#[derive(Queryable, Selectable, Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wallets {
    pub user_id: i32,
    pub sol_addr: String,
}

#[derive(Queryable, Selectable, Debug, Insertable, Clone)]
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

#[derive(Queryable, Selectable, Debug, Insertable, Clone)]
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

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone, Default)]
#[diesel(table_name = crate::schema::test_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TestCases {
    pub test_case_id: i32,
    pub question_id: i32,
    pub test_inputs: String,  // stringified json
    pub test_outputs: String, // stringified json
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone)]
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

// ------------------------------- insertable models ----------------------------
// the following models are not containing the primary key fields, making able the fns to insert the values without the pks.

#[derive(Queryable, Selectable, Debug, Insertable, Clone)]
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

#[derive(Queryable, Selectable, Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IQuestions {
    // pub question_id: i32, // pk
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    // @dev because the deadlines and the creation times are set in the same timezone they are comparable and no conflict is expected
    // pub creation_time: NaiveDateTime, // will be the default value defined in the table
    pub deadline: NaiveDateTime,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub category: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone)]
#[diesel(table_name = crate::schema::test_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ITestCases {
    // pub test_case_id: i32, // pk
    pub question_id: i32,
    pub test_inputs: String,
    pub test_outputs: String,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone)]
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

// ------------------------------- queryable models ----------------------------
// the following models will provide the dynamic inputs for the getter functions

pub struct QResponses {
    pub response_id: Option<i32>,
    pub question_id: Option<i32>,
    pub daredevil_id: Option<i32>,
    // mod 1 = Some, None, None => search by the response id.
    // mod 2 = None, Some, Some => search by the daredevil and related question id.
}

impl QResponses {
    pub fn is_correct_structures(instance: &QResponses) -> bool {
        if (instance.question_id.is_some()
            && !instance.response_id.is_none()
            && !instance.daredevil_id.is_none())
            | (instance.question_id.is_none()
                && !instance.response_id.is_some()
                && !instance.daredevil_id.is_some())
        {
            true
        } else {
            false
        }
    }
}
#[derive(Clone)]
pub enum Categories {
    All,
    SolanaPrograms,
    Rust,
}
impl Categories {
    pub fn to_string(category: Option<&Self>) -> String {
        match category {
            Some(cat) => match cat {
                Self::All => "All".to_string(),
                Self::Rust => "Rust".to_string(),
                Self::SolanaPrograms => "SolanaPrograms".to_string(),
            },
            _ => "All".to_string(),
        }
    }
}
pub struct QQuestions<'a> {
    pub question_id: Option<i32>,
    pub question_title: Option<&'a str>,
    pub rival_id: Option<i32>,
    pub question_category: Option<Categories>,
    // mod 1 = Some, None, None, None => search by the question id.
    // mod 2 = None, Some, Some, None => search by the rival and related question title.
    // mod 3 = None, None, None, Some => get all questions or a certain category of the questions.
}

// un-optimized -> must be done using the match
impl QQuestions<'_> {
    pub fn is_correct_structures(instance: &QQuestions) -> bool {
        if (instance.question_id.is_some()
            && !instance.question_title.is_none()
            && !instance.rival_id.is_none()
            && !instance.question_category.is_none())
            | (instance.question_id.is_none()
                && !instance.question_title.is_some()
                && !instance.rival_id.is_some()
                && !instance.question_category.is_none())
            | (instance.question_id.is_none()
                && !instance.question_title.is_none()
                && !instance.rival_id.is_none()
                && !instance.question_category.is_some())
        {
            true
        } else {
            false
        }
    }
}

// ------------------------------- updatable models ----------------------------
// the following models will provide the dynamic inputs for the getter functions
pub struct UUser<'a> {
    pub old_username_or_id: &'a str,
    pub new_email: &'a str,
    pub new_password: &'a str,
    pub new_username: &'a str,
    pub editor: &'a str,
}

pub struct UQuestion<'a> {
    pub editor: &'a str,
    pub rival_id: &'a str,
    pub old_question_title: &'a str, // fetched from fe
    pub question_title: &'a str,     // fetched from fe
    pub question_body: &'a str,      // fetched from fe
    pub deadline: &'a str,           // fetched from fe
    pub question_status: i32,        // fetched from fe
    pub daredevil: Option<i32>,      // fetched from fe
    pub category: &'a str,           // fetched from fe
    pub test_inputs: &'a str,        // if empty will not be updated
    pub test_outputs: &'a str,       // if empty will not be updated
}

pub struct UWallets<'a> {
    pub editor: &'a str,
    pub username_or_id: &'a str,
    pub new_sol_addr: &'a str,
}

// ---------------------------------------------------------- removable models ---------------------------------------------------------
// the following models will provide the simple editor user id and certain unique single input for removing the values from the database
pub struct RUsers<'a> {
    pub remover: &'a str,
    pub username_or_id: &'a str,
}
