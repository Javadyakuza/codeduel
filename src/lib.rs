#![recursion_limit = "256"]
pub mod api_models;
pub mod db_models;
pub mod schema;

use crate::db_models::{
    IQuestions, IResponses, ITestCases, IUsers, Questions, Responses, TestCases, Users, Wallets,
};
use crate::schema::{questions, responses, test_cases, users, wallets};
use chrono::Local;
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use schema::{
    questions::dsl::*, responses::dsl::*, test_cases::dsl::*, users::dsl::*, wallets::dsl::*,
};

pub use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn establish_connection() -> PgConnection {
    // loading the env vars into the current scope
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// general ** setters
pub fn add_new_user(
    conn: &mut PgConnection,
    _new_user: &IUsers,
) -> Result<Users, Box<dyn std::error::Error>> {
    // inserting the new user
    match diesel::insert_into(users::table)
        .values(_new_user)
        .returning(Users::as_returning())
        .get_result(conn)
    {
        Ok(nu) => Ok(nu),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn add_user_wallet(
    conn: &mut PgConnection,
    _new_wallet: &Wallets,
) -> Result<Wallets, Box<dyn std::error::Error>> {
    // inserting the new user
    match diesel::insert_into(wallets::table)
        .values(_new_wallet)
        .returning(Wallets::as_returning())
        .get_result(conn)
    {
        Ok(nw) => Ok(nw),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn add_new_question(
    conn: &mut PgConnection,
    _new_question: &IQuestions,
    _test_cases: &ITestCases,
) -> Result<(Questions, TestCases), Box<dyn std::error::Error>> {
    // inserting the new user
    match diesel::insert_into(questions::table)
        .values(_new_question)
        .returning(Questions::as_returning())
        .get_result(conn)
    {
        Ok(nq) => {
            match diesel::insert_into(test_cases::table)
                .values(_test_cases)
                .returning(TestCases::as_returning())
                .get_result(conn)
            {
                Ok(nt) => Ok((nq, nt)),
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{:?}", e),
                    )))
                }
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn add_response(
    conn: &mut PgConnection,
    _new_response: &IResponses,
) -> Result<Responses, Box<dyn std::error::Error>> {
    // inserting the new user
    match diesel::insert_into(responses::table)
        .values(_new_response)
        .returning(Responses::as_returning())
        .get_result(conn)
    {
        Ok(nr) => Ok(nr),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}
