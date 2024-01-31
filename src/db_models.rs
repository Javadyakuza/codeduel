// use std::{ops::Deref, str::FromStr};

use diesel::prelude::*;
// use merge_derivable;
use crate::api_models::{EpInQuestions, EpQuQuestions};
use chrono::NaiveDateTime;
use rocket::{
    http::Header,
    response::{self, Responder},
    FromForm, Request, Response,
};
// use rocket::{http::RawStr, request::FromFormValue};
use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;
pub const OPEN_UNSOLVED: i32 = 1;
pub const OPEN_SOLVED: i32 = 2;
pub const CLOSED_UNSOLVED: i32 = 1;
pub const CLOSED_SOLVED: i32 = 1;

// ------------------------------- general models ----------------------------
// the following models will represent the actual schema of the tables in terms of the rust structs.
#[derive(FromForm, Queryable, Selectable, Debug, Insertable, Clone, Serialize, Iterable)]
#[diesel(table_name = crate::schema::wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wallets {
    pub user_id: i32,
    pub sol_addr: String,
}

#[derive(Queryable, Selectable, Debug, Insertable, Clone, Iterable, Serialize)]
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

#[derive(Queryable, Selectable, Debug, Insertable, Clone, Serialize, Iterable)]
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
    pub reward: i32,
    pub prize_pool: i32,
    pub entrance_fee: f32,
    pub category: String,
}

#[derive(
    Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone, Default, Iterable,
)]
#[diesel(table_name = crate::schema::test_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TestCases {
    pub test_case_id: i32,
    pub question_id: i32,
    pub executable_solution: String,
    pub solution_executer: String,
    pub test_inputs: String,  // stringified json
    pub test_outputs: String, // stringified json
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone, Iterable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub total_payed: i32,
    pub total_claimed: i32,
    pub total_unclaimed: i32,
}
// impl<'r, 'o: 'r> Responder<'r, 'o> for Users {
//     fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
//         // log `self` to your favored error tracker, e.g.
//         // sentry::capture_error(&self);

//         match self {
//             // in our simplistic example, we're happy to respond with the default 500 responder in all cases
//             _ => Ok(Response::build_from(self).finalize()),
//         }
//     }
// }
// ------------------------------- insertable models ----------------------------
// the following models are not containing the primary key fields, making able the fns to insert the values without the pks.

#[derive(FromForm, Queryable, Selectable, Debug, Insertable, Clone, Iterable)]
#[diesel(table_name = crate::schema::responses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IResponses {
    // pub response_id: i32, // pk
    pub daredevil_id: i32,
    pub question_id: i32,
    pub response_code: String,
    pub correctness: bool,
    // pub creation_time: NaiveDateTime, calculated by the db
}

#[derive(Queryable, Selectable, Debug, Insertable, Clone, Serialize, Iterable)]
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
    pub reward: i32,
    pub entrance_fee: f32,
}
impl IQuestions {
    pub fn from_ep_in_question(ep_q: EpInQuestions) -> Result<Self, Box<dyn std::error::Error>> {
        let _deadline: NaiveDateTime;
        match NaiveDateTime::parse_from_str(ep_q.deadline.as_str(), "%Y-%m-%d %H:%M:%S") {
            Ok(d) => _deadline = d,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "question not found !",
                )))
            }
        }
        Ok(Self {
            rival_id: ep_q.rival_id,
            question_title: ep_q.question_title,
            question_body: ep_q.question_body,
            deadline: _deadline,
            question_status: ep_q.question_status,
            daredevil: ep_q.daredevil,
            category: ep_q.category,
            reward: ep_q.reward,
            entrance_fee: ep_q.entrance_fee,
        })
    }
}
#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone, Iterable)]
#[diesel(table_name = crate::schema::test_cases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ITestCases {
    // pub test_case_id: i32, // pk
    pub question_id: i32,
    pub executable_solution: String,
    pub solution_executer: String,
    pub test_inputs: String,
    pub test_outputs: String,
}

#[derive(
    FromForm, Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, Clone, Iterable,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IUsers {
    // pub user_id: i32, // pk
    pub email: String,
    pub username: String,
    pub password: String,
}

// ------------------------------- queryable models ----------------------------
// the following models will provide the dynamic inputs for the getter functions

pub struct QResponses {
    pub response_id: Option<i32>,
    pub question_id: Option<i32>,
    pub daredevil_id: Option<i32>,
    // mod 1 = Some, None, None => search by the response id.
    // mod 2 = None, Some, Some => search by the daredevil and related question id.
    // mod 3 = None, None, Some => get all of the responses of the daredevil.
}

impl QResponses {
    pub fn is_correct_structures(instance: &QResponses) -> i32 {
        match (
            instance.question_id.is_some(),
            instance.response_id.is_some(),
            instance.daredevil_id.is_some(),
        ) {
            // Case 1: All fields are present
            (true, false, false) => 1,

            // Case 2: Question ID and Daredevil ID are present
            (false, true, true) => 2,

            // Case 3: Response ID and Daredevil ID are present
            (false, false, true) => 3,

            // None of the fields are present
            _ => 0,
        }
    }
}

#[derive(Clone, Serialize, Debug)]
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
    pub fn from_string(category: &str) -> Categories {
        match category {
            "All" | "all" => Self::All,
            "Rust" | "rust" => Self::Rust,
            "SolanaPrograms" | "solanaprograms" => Self::SolanaPrograms,

            _ => Self::All,
        }
    }
}
#[derive(Default, Serialize, Debug, Iterable)]
pub struct QQuestions {
    pub question_id: Option<i32>,
    pub question_title: Option<String>, // the queryable title does not have any fixed length.
    pub rival_id: Option<i32>,
    pub question_category: Option<Categories>,
    // mod 1 = Some, None, None, None => search by the question id.
    // mod 2 = None, Some, Some, None => search by the rival and related question title.
    // mod 3 = None, None, Some, None => get all of the rival questions
    // mod 4 = None, None, None, Some => get all questions or a certain category of the questions.
}

impl QQuestions {
    pub fn is_correct_structures(instance: &QQuestions) -> i32 {
        match (
            instance.question_id.is_some(),
            instance.question_title.is_some(),
            instance.rival_id.is_some(),
            instance.question_category.is_some(),
        ) {
            // Case 1: All fields are present
            (true, false, false, true) => 1,

            // Case 2: Question ID, Rival ID, and Question Category are present
            (false, true, true, true) => 2,

            // Case 3: Question Title, Rival ID, and Question Category are present
            (false, false, true, true) => 3,

            // Case 4: Question Title and Rival ID are present
            (false, false, false, true) => 4,

            // None of the fields are present
            _ => 0,
        }
    }
    pub fn clone(self) -> Self {
        Self {
            question_id: self.question_id,
            question_title: self.question_title.clone(),
            rival_id: self.rival_id,
            question_category: self.question_category.clone(),
        }
    }
    pub fn build_from_ep(ep_question: &EpQuQuestions) -> Self {
        let mut qq: QQuestions = Default::default();
        if ep_question.question_id == 0 {
            qq.question_id = None;
        } else {
            qq.question_id = Some(ep_question.question_id);
        }
        if &ep_question.question_title == "" {
            qq.question_title = None;
        } else {
            qq.question_title = Some(ep_question.question_title.clone());
        }
        if ep_question.rival_id == 0 {
            qq.rival_id = None;
        } else {
            qq.rival_id = Some(ep_question.rival_id);
        }
        if ep_question.question_category == "" {
            qq.question_category = Some(Categories::All);
        } else {
            qq.question_category = Some(Categories::from_string(
                ep_question.question_category.as_str(),
            ));
        }
        println!("{:?}", qq);
        Self {
            question_id: qq.question_id,
            question_title: qq.question_title,
            rival_id: qq.rival_id,
            question_category: qq.question_category,
        }
    }
}

// ------------------------------- updatable models ----------------------------
// the following models will provide the dynamic inputs for the getter functions
#[derive(FromForm, Debug)]
pub struct UUser {
    pub old_username_or_id: String,
    pub new_email: String,
    pub new_password: String,
    pub new_username: String,
    pub new_total_payed: i32,
    pub new_total_claimed: i32,
    pub new_total_unclaimed: i32,
    pub editor: String,
}

// @notice "fetched from fe" means the same old values will be fetched and sent to backend by the front end application
#[derive(FromForm, Debug)]
pub struct UQuestion {
    pub editor: String,
    pub rival_id: String,
    pub old_question_title: String, // fetched from fe
    pub question_title: String,     // fetched from fe
    pub question_body: String,      // fetched from fe
    pub deadline: String,           // checked in the backend
    pub question_status: i32,       // fetched from fe
    pub daredevil: i32,             // fetched from fe, zero is considered as no daredevil
    pub prize_pool: i32,            // fetched from fe
    pub category: String,           // fetched from fe
    pub test_inputs: String,        // if empty will not be updated
    pub test_outputs: String,       // if empty will not be updated
}

#[derive(FromForm, Debug)]
pub struct UWallets {
    pub editor: String,
    pub username_or_id: String,
    pub new_sol_addr: String,
}

// ---------------------------------------------------------- removable models ---------------------------------------------------------
// the following models will provide the simple editor user id and certain unique single input for removing the values from the database
#[derive(FromForm, Debug)]

pub struct RUsers {
    pub remover: String,
    pub username_or_id: String,
}

#[derive(FromForm, Debug)]

pub struct RQuestions {
    pub remover: String,
    pub rival_id: String,
    pub question_title: String, // fetched from fe
}

