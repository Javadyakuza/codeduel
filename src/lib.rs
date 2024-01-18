#![recursion_limit = "256"]
pub mod api_models;
pub mod db_models;
pub mod schema;

use crate::db_models::{Questions, Responses, TestCases, Users, Wallets};
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
