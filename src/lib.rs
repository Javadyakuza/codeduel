#![recursion_limit = "256"]
pub mod api_models;
pub mod db_models;
pub mod schema;
use crate::db_models::{
    IQuestions, IResponses, ITestCases, IUsers, Questions, Responses, TestCases, Users, Wallets,
};
use crate::schema::{questions, responses, test_cases, users, wallets};
use chrono::NaiveDateTime;
use db_models::{
    Categories, QQuestions, QResponses, RQuestions, RUsers, UQuestion, UUser, UWallets,
};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use schema::{
    questions::dsl::*, responses::dsl::*, test_cases::dsl::*, users::dsl::*, wallets::dsl::*,
};

pub use std::env;

pub fn establish_connection() -> PgConnection {
    // loading the env vars into the current scope
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// general ** setters
// unchecked - un-optimized - unchecked word casing (all must be saved in the lowercase)
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

// unchecked - un-optimized
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

// unchecked - un-optimized
pub fn add_new_question(
    conn: &mut PgConnection,
    _new_question: &IQuestions,
    _test_cases: &mut ITestCases,
) -> Result<(Questions, TestCases), Box<dyn std::error::Error>> {
    // inserting the new user
    match diesel::insert_into(questions::table)
        .values(_new_question)
        .returning(Questions::as_returning())
        .get_result(conn)
    {
        Ok(nq) => {
            _test_cases.question_id = nq.question_id;
            match diesel::insert_into(test_cases::table)
                .values(_test_cases.to_owned())
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
    // todo!("formatting the question title");
}

// unchecked - un-optimized
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

// general ** getter
pub fn get_user(
    _conn: &mut PgConnection,
    _username_or_id: &str,
) -> Result<Users, Box<dyn std::error::Error>> {
    // fetching the user id if the user name was provided
    match _username_or_id.parse::<i32>() {
        Ok(ui) => {
            let tmp_users: Vec<Users> = users
                .filter(users::user_id.eq(ui))
                .select(Users::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_users.len() == 0 {
                // chat room id doesn't exists
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "user id not found !",
                )));
            } else if tmp_users[0].username.as_str() == "deleted" {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "user is deleted",
                )));
            } else {
                return Ok(tmp_users[0].to_owned());
            }
        }
        Err(_) => {
            let tmp_users: Vec<Users> = users
                .filter(users::username.eq(_username_or_id))
                .select(Users::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_users.len() == 0 {
                // chat room id doesn't exists
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "username not found !",
                )));
            } else if tmp_users[0].username.as_str() == "deleted" {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "user is deleted",
                )));
            } else {
                return Ok(tmp_users[0].to_owned());
            }
        }
    }
}

pub fn get_response(
    _conn: &mut PgConnection,
    _query_struct: &QResponses,
) -> Result<Vec<Responses>, Box<dyn std::error::Error>> {
    // checking the format of the struct
    let _mod = QResponses::is_correct_structures(_query_struct);
    if _mod == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "the response query format is wrong ! \n
            supported formats: \n
            1 - Some(question_id), None(response_id), None(daredevil_id) \n
            2 - None(question_id), Some(response_id), Some(daredevil_id)",
        )));
    } else {
        if _mod == 1 {
            let tmp_responses: Vec<Responses> = responses
                .filter(responses::response_id.eq(_query_struct.response_id.unwrap())) // panic impossible
                .select(Responses::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_responses.len() == 0 {
                // response doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no responses found !",
                )))
            } else {
                Ok(tmp_responses.to_owned())
            }
        } else if _mod == 2 {
            // searching by the daredevil id
            let tmp_responses: Vec<Responses> = responses
                .filter(responses::question_id.eq(_query_struct.question_id.unwrap())) // panic impossible
                .filter(responses::daredevil_id.eq(_query_struct.daredevil_id.unwrap())) // panic impossible
                .select(Responses::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_responses.len() == 0 {
                // response doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no responses found !",
                )))
            } else {
                Ok(tmp_responses.to_owned())
            }
        } else {
            // getting all of the responses of the daredevil
            // searching by the daredevil id
            let tmp_responses: Vec<Responses> = responses
                .filter(responses::daredevil_id.eq(_query_struct.daredevil_id.unwrap())) // panic impossible
                .select(Responses::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_responses.len() == 0 {
                // response doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no responses found !",
                )))
            } else {
                Ok(tmp_responses.to_owned())
            }
        }
    }
}

pub fn get_question(
    _conn: &mut PgConnection,
    _query_struct: &QQuestions,
) -> Result<Vec<Questions>, Box<dyn std::error::Error>> {
    // todo!("formatting the question title");
    // checking the format of the struct
    let _mod = QQuestions::is_correct_structures(_query_struct);
    if _mod == 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "the question query format is wrong ! \n
            supported formats: \n
            1 - Some(question_id), None(question_title), None(rival_id), None(question_category) \n
            2 - None(question_id), Some(question_title), Some(rival_id), None(question_category) \n 
            3 - None(question_id), None(question_title), None(rival_id), Some(question_category)",
        )));
    } else {
        if _mod == 1 {
            // searching by the question id
            let tmp_questions: Vec<Questions> = questions
                .filter(questions::question_id.eq(_query_struct.question_id.unwrap())) // panic impossible
                .select(Questions::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_questions.len() == 0 {
                // question doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "question not found !",
                )))
            } else {
                Ok(tmp_questions.to_owned())
            }
        } else if _mod == 2 {
            // searching by the rival id and title of the question
            let tmp_questions: Vec<Questions> = questions
                .filter(questions::rival_id.eq(_query_struct.rival_id.unwrap())) // panic impossible
                .filter(
                    questions::question_title.eq(_query_struct.question_title.as_ref().unwrap()),
                ) // panic impossible
                .select(Questions::as_select())
                .load(_conn)
                .unwrap_or(vec![]);

            if tmp_questions.len() == 0 {
                // response doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "question not found !",
                )))
            } else {
                Ok(tmp_questions.to_owned())
            }
        } else if _mod == 3 {
            // searching by the category
            let tmp_questions: Vec<Questions> = questions
                .filter(questions::rival_id.eq(_query_struct.rival_id.unwrap())) // panic impossible
                .select(Questions::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_questions.len() == 0 {
                // question doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "question not found !",
                )))
            } else {
                Ok(tmp_questions.to_owned())
            }
        } else {
            // searching by the category
            let tmp_questions: Vec<Questions> = questions
                .filter(questions::category.eq(Categories::to_string(
                    _query_struct.question_category.as_ref(),
                ))) // panic impossible
                .select(Questions::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_questions.len() == 0 {
                // question doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "question not found !",
                )))
            } else {
                Ok(tmp_questions.to_owned())
            }
        }
    }
}

pub fn get_test_cases(
    _conn: &mut PgConnection,
    _question_id: i32,
) -> Result<TestCases, Box<dyn std::error::Error>> {
    let tmp_questions_tcs: Vec<TestCases> = test_cases
        .filter(test_cases::question_id.eq(_question_id)) // panic impossible
        .select(TestCases::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if tmp_questions_tcs.len() == 0 {
        // question doesn't exists
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "question not found !",
        )))
    } else {
        Ok(tmp_questions_tcs[0].to_owned())
    }
}

// // general ** updaters
pub fn update_user(
    _conn: &mut PgConnection,
    _new_user_info: &UUser,
) -> Result<Users, Box<dyn std::error::Error>> {
    // checking the editor authority for editing the user info
    let user_old_info: Users;
    match check_authority(
        _conn,
        _new_user_info.editor,
        _new_user_info.old_username_or_id,
    ) {
        Ok(ui) => user_old_info = ui,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }

    // updating the chat room info
    match diesel::update(users.filter(users::user_id.eq(user_old_info.user_id)))
        .set((
            email.eq(&_new_user_info.new_email),
            password.eq(&_new_user_info.new_password),
            username.eq(&_new_user_info.new_username),
        ))
        .returning(Users::as_returning())
        .get_result(_conn)
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

pub fn update_question(
    _conn: &mut PgConnection,
    _new_question_info: &UQuestion,
) -> Result<Questions, Box<dyn std::error::Error>> {
    // checking the editor authority for editing the question
    let user_old_info: Users;
    match check_authority(
        _conn,
        _new_question_info.editor,
        _new_question_info.rival_id,
    ) {
        Ok(ui) => user_old_info = ui,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }

    let _question: Vec<Questions>;

    match get_question(
        _conn,
        &QQuestions {
            question_id: None,
            question_title: Some(_new_question_info.question_title.to_string()),
            rival_id: Some(user_old_info.user_id),
            question_category: None,
        },
    ) {
        Ok(qi) => _question = qi,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(" {:?}", e),
            )))
        }
    }

    // preparing the deadline, the deadline can only be increased
    let _deadline: NaiveDateTime;
    match NaiveDateTime::parse_from_str(&_new_question_info.deadline, "%Y-%m-%d %H:%M:%S") {
        Ok(ndt) => {
            if ndt < _question[0].deadline {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("cant not decrease the deadline"),
                )));
            } else {
                _deadline = ndt;
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("couldn't parse the date time {:?}", e),
            )))
        }
    }
    // the new prize pool amount can only be increased
    if _question[0].prize_pool < _new_question_info.prize_pool {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "prize pool amount can only be increased !",
        )));
    }
    // preparing the test-cases
    let mut old_tcs: TestCases = Default::default();
    match get_test_cases(_conn, _question[0].question_id) {
        Ok(tc) => {
            if _new_question_info.test_inputs == "" {
                old_tcs.test_inputs = tc.test_inputs
            } else {
                old_tcs.test_inputs = _new_question_info.test_inputs.to_string()
            }
            if _new_question_info.test_outputs == "" {
                old_tcs.test_outputs = tc.test_outputs
            } else {
                old_tcs.test_outputs = _new_question_info.test_outputs.to_string()
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(" {:?}", e),
            )))
        }
    }
    // updating the questions table
    match diesel::update(questions.filter(questions::question_id.eq(_question[0].question_id)))
        .set((
            question_title.eq(&_new_question_info.question_title), // check unnecessary
            question_body.eq(&_new_question_info.question_body),   // check unnecessary
            deadline.eq(&_deadline),                               // checked
            question_status.eq(&_new_question_info.question_status), // check unnecessary
            prize_pool.eq(_new_question_info.prize_pool),          //checked
            daredevil.eq(&_new_question_info.daredevil),           // check unnecessary
            category.eq(&_new_question_info.category),             // check unnecessary
        ))
        .returning(Questions::as_returning())
        .get_result(_conn)
    {
        Ok(nq) => {
            match diesel::update(test_cases.filter(test_cases::question_id.eq(nq.question_id)))
                .set((
                    test_inputs.eq(&old_tcs.test_inputs),   // checked
                    test_outputs.eq(&old_tcs.test_outputs), // checked
                ))
                .returning(TestCases::as_returning())
                .get_result(_conn)
            {
                Ok(_) => Ok(nq),
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

// @dev this function will be used in case of the private key leakage
pub fn update_user_wallet(
    _conn: &mut PgConnection,
    _new_user_wallet_info: &UWallets,
) -> Result<Wallets, Box<dyn std::error::Error>> {
    // checking the editor authority for editing the user info
    let user_old_info: Users;
    match check_authority(
        _conn,
        _new_user_wallet_info.editor,
        _new_user_wallet_info.username_or_id,
    ) {
        Ok(ui) => user_old_info = ui,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }

    // updating the chat room info
    match diesel::update(wallets.filter(wallets::user_id.eq(user_old_info.user_id)))
        .set((sol_addr.eq(&_new_user_wallet_info.new_sol_addr),))
        .returning(Wallets::as_returning())
        .get_result(_conn)
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

// ------------- @dev (the responses will not get updated once they are submitted)
// ------------- the multiple responses to a single question will be added as a feature in the future.{see ./future_features.md}

// // general removers
pub fn delete_user(
    _conn: &mut PgConnection,
    _user_info: &RUsers,
) -> Result<bool, Box<dyn std::error::Error>> {
    // checking the authority of the remover
    let user_old_info: Users;
    match check_authority(_conn, _user_info.remover, _user_info.username_or_id) {
        Ok(ui) => user_old_info = ui,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }

    // the questions and the responses of the user wont be deleted
    // the user can not remove the designed questions, the user must wait until the highest deadline amount.
    // the user will not be deleted, its user information will be changed to the "deleted", it is being referenced by other tables fields.
    if let Ok(qs) = get_question(
        _conn,
        &QQuestions {
            question_id: None,
            question_title: None,
            rival_id: Some(user_old_info.user_id),
            question_category: None,
        },
    ) {
        for q in qs.iter() {
            if q.question_status < 3 {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!("rival has a open question, must wait until the deadline"),
                )));
            }
        }
    };
    // user must claim all of the unclaimed tokens from the prize pool
    if user_old_info.total_unclaimed != 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "user must claim all of the un-claimed tokens of the reserve pool",
        )));
    }
    if let Err(e) = delete_user_wallet(_conn, _user_info) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        )));
    }
    match update_user(
        _conn,
        &UUser {
            old_username_or_id: user_old_info.username.as_str(),
            new_email: "deleted",
            new_password: "deleted",
            new_username: "deleted",
            new_total_payed: user_old_info.total_payed, // same as before
            new_total_claimed: user_old_info.total_claimed, // same as before
            new_total_unclaimed: user_old_info.total_unclaimed, // same as before
            editor: user_old_info.username.as_str(),
        },
    ) {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't delete the user \n {:?}", e),
            )))
        }
    }
}

pub fn delete_user_wallet(
    _conn: &mut PgConnection,
    _user_info: &RUsers,
) -> Result<bool, Box<dyn std::error::Error>> {
    // checking the authority of the remover
    let user_old_info: Users;
    match check_authority(_conn, _user_info.remover, _user_info.username_or_id) {
        Ok(ui) => user_old_info = ui,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }

    match diesel::delete(wallets.filter(wallets::user_id.eq(user_old_info.user_id))).execute(_conn)
    {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't delete the user wallet \n {:?}", e),
            )))
        }
    }
}

pub fn delete_question(
    _conn: &mut PgConnection,
    _question_info: &RQuestions,
) -> Result<bool, Box<dyn std::error::Error>> {
    // checking the authority of the remover
    let user_old_info: Users;
    match check_authority(_conn, _question_info.remover, _question_info.rival_id) {
        Ok(ui) => user_old_info = ui,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
    let _question: Vec<Questions>;
    match get_question(
        _conn,
        &QQuestions {
            question_id: None,
            question_title: Some(_question_info.question_title.to_string()),
            rival_id: Some(user_old_info.user_id),
            question_category: None,
        },
    ) {
        Ok(q) => {
            if q[0].prize_pool != 0 {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("the question has already obtained tokens in the prize pool, question can not be deleted"),
                )));
            }
            _question = q
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }

    // checking if the question is not having any tokens obtained
    match diesel::delete(questions.filter(questions::question_id.eq(_question[0].question_id)))
        .execute(_conn)
    {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't delete the question \n {:?}", e),
            )))
        }
    }
}

// @dev (responses can not be deleted)

// // custom ** getter
pub fn check_authority(
    _conn: &mut PgConnection,
    _1: &str,
    _2: &str,
) -> Result<Users, Box<dyn std::error::Error>> {
    match get_user(_conn, _1) {
        Ok(eu) => match get_user(_conn, _1) {
            Ok(ou) => {
                if eu.user_id == ou.user_id {
                    Ok(ou)
                } else {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        format!("edit permission denied for user id {}", _1),
                    )));
                }
            }
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{:?}", e),
                )))
            }
        },
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }
}
