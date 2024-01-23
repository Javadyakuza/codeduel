#![recursion_limit = "256"]
pub mod api_models;
pub mod db_models;
pub mod schema;
use crate::db_models::{
    IQuestions, IResponses, ITestCases, IUsers, Questions, Responses, TestCases, Users, Wallets,
};
use crate::schema::{questions, responses, test_cases, users, wallets};
use chrono::{Local, NaiveDate, NaiveDateTime};
use db_models::{Categories, QQuestions, QResponses, RUsers, UQuestion, UUser, UWallets};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use rocket::Response;
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
            } else {
                return Ok(tmp_users[0].to_owned());
            }
        }
        Err(e) => {
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
            } else {
                return Ok(tmp_users[0].to_owned());
            }
        }
    }
}

pub fn get_response(
    _conn: &mut PgConnection,
    _query_struct: &QResponses,
) -> Result<Responses, Box<dyn std::error::Error>> {
    // checking the format of the struct
    if !QResponses::is_correct_structures(_query_struct) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "the response query format is wrong ! \n
            supported formats: \n
            1 - Some(question_id), None(response_id), None(daredevil_id) \n
            2 - None(question_id), Some(response_id), Some(daredevil_id)",
        )));
    } else {
        if _query_struct.question_id.is_none() {
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
                    "response not found !",
                )))
            } else {
                Ok(tmp_responses[0].to_owned())
            }
        } else {
            let tmp_responses: Vec<Responses> = responses
                .filter(responses::response_id.eq(_query_struct.response_id.unwrap())) // panic impossible
                .select(Responses::as_select())
                .load(_conn)
                .unwrap_or(vec![]);
            if tmp_responses.len() == 0 {
                // response doesn't exists
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "response not found !",
                )))
            } else {
                Ok(tmp_responses[0].to_owned())
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
            3 - None(question_id), None(question_titel), None(rival_id), Some(question_category)",
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
                .filter(questions::question_title.eq(_query_struct.question_title.unwrap())) // panic impossible
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

    let _question: Questions;

    match get_question(
        _conn,
        &QQuestions {
            question_id: None,
            question_title: Some(_new_question_info.question_title),
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
            if ndt < _question.deadline {
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
    // preparing the testcases
    let mut old_tcs: TestCases = Default::default();
    match get_test_cases(_conn, _question.question_id) {
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
    match diesel::update(questions.filter(questions::question_id.eq(_question.question_id)))
        .set((
            question_title.eq(&_new_question_info.question_title),
            question_body.eq(&_new_question_info.question_body),
            deadline.eq(&_deadline),
            question_status.eq(&_new_question_info.question_status),
            daredevil.eq(&_new_question_info.daredevil),
            category.eq(&_new_question_info.category),
        ))
        .returning(Questions::as_returning())
        .get_result(_conn)
    {
        Ok(nq) => {
            match diesel::update(test_cases.filter(test_cases::question_id.eq(nq.question_id)))
                .set((
                    test_inputs.eq(&old_tcs.test_inputs),
                    test_outputs.eq(&old_tcs.test_outputs),
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
    let _question: Questions;
    match get_question(
        _conn,
        &QQuestions {
            question_id: None,
            question_title: None,
            rival_id: Some(user_old_info.user_id),
            question_category: None,
        },
    ) {
        Ok(q) => _question = q,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("user must wait until the highest dead line {:?}", e),
            )))
        }
    }

    Ok(true)
}

// // custom ** setters
// pub fn add_new_group_chat_room(
//     _conn: &mut PgConnection,
//     _chat_room_info: &ChatRooms,
//     group_owner_username: &String,
//     group_members: Vec<String>,
// ) -> Result<QChatRooms, Box<dyn std::error::Error>> {
//     let group_owner_id: i32;
//     match get_user_with_username(_conn, group_owner_username) {
//         Ok(res) => group_owner_id = res.user_id,
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     // creating the chat room
//     let new_chat_room;
//     match diesel::insert_into(chat_rooms)
//         .values(_chat_room_info)
//         .returning(QChatRooms::as_returning())
//         .get_result(_conn)
//     {
//         Ok(res) => new_chat_room = res,
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!("{:?}", e),
//             )))
//         }
//     }
//     // adding the owner to the participants
//     let owner = ChatRoomParticipants {
//         user_id: group_owner_id,
//         chat_room_id: new_chat_room.chat_room_id,
//         is_admin: true,
//     };
//     if let Err(e) = diesel::insert_into(chat_room_participants)
//         .values(&owner)
//         .returning(ChatRoomParticipants::as_returning())
//         .get_result(_conn)
//     {
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             format!("{:?}", e),
//         )));
//     }
//     // adding members if any specified
//     if group_members.len() > 0 {
//         let mut group_members_up: Vec<ChatRoomParticipants> = Vec::new();
//         for member in group_members {
//             let _member_id: i32;
//             match get_user_with_username(_conn, member.as_str()) {
//                 Ok(res) => _member_id = res.user_id,
//                 Err(e) => {
//                     return Err(Box::new(std::io::Error::new(
//                         std::io::ErrorKind::NotFound,
//                         format!("{:?}", e),
//                     )))
//                 }
//             }
//             if _member_id != group_owner_id {
//                 group_members_up.push(ChatRoomParticipants {
//                     user_id: _member_id,
//                     chat_room_id: new_chat_room.chat_room_id,
//                     is_admin: false,
//                 });
//             }
//         }
//         if let Err(e) = diesel::insert_into(chat_room_participants::table)
//             .values(&group_members_up)
//             .returning(ChatRoomParticipants::as_returning())
//             .get_result(_conn)
//         {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!("{:?}", e),
//             )));
//         }
//     }
//     Ok(new_chat_room)
// }

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
                        format!("edit premission denied for user id {}", _1),
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

// // custom ** updaters
// pub fn update_group_chat_room_info(
//     _conn: &mut PgConnection,
//     old_chat_room_name: &String,
//     new_chat_room_info: &UpdatableChatRooms,
//     editor_username: &String,
// ) -> Result<QChatRooms, Box<dyn std::error::Error>> {
//     let _chat_room_id: i32;
//     match get_group_chat_by_name(_conn, old_chat_room_name) {
//         Ok(res) => _chat_room_id = res.chat_room_id,
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     let editor_user_id: i32;
//     match get_user_with_username(_conn, &editor_username) {
//         Ok(res) => editor_user_id = res.user_id,
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     match get_group_owner_by_id(_conn, _chat_room_id) {
//         Ok(res) => {
//             if editor_user_id != res {
//                 return Err(Box::new(std::io::Error::new(
//                     std::io::ErrorKind::PermissionDenied,
//                     format!(
//                         "user id {} is not allowed to edit the group info",
//                         editor_user_id
//                     ),
//                 )));
//             }
//         }
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     // updating the chat room info
//     match diesel::update(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
//         .set((
//             room_name.eq(&new_chat_room_info.room_name),
//             room_description.eq(&new_chat_room_info.room_description),
//         ))
//         .returning(QChatRooms::as_returning())
//         .get_result(_conn)
//     {
//         Ok(_) => Ok(get_group_chat_by_name(_conn, &new_chat_room_info.room_name).unwrap()),
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!("{:?}", e),
//             )))
//         }
//     }
// }

// // custom removers
// pub fn delete_group_chat_room(
//     _conn: &mut PgConnection,
//     _chat_room_name: &String,
//     remover_username: &String,
// ) -> Result<bool, Box<dyn std::error::Error>> {
//     let remover_user_id: i32;
//     match get_user_with_username(_conn, remover_username) {
//         Ok(res) => remover_user_id = res.user_id,
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     let _chat_room_id: i32;
//     match get_group_chat_by_name(_conn, _chat_room_name) {
//         Ok(res) => _chat_room_id = res.chat_room_id,
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     match get_group_owner_by_id(_conn, _chat_room_id) {
//         Ok(res) => {
//             if remover_user_id != res {
//                 return Err(Box::new(std::io::Error::new(
//                     std::io::ErrorKind::PermissionDenied,
//                     format!(
//                         "user id {} is not allowed to delete the group",
//                         remover_user_id
//                     ),
//                 )));
//             }
//         }
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 format!("{:?}", e),
//             )))
//         }
//     }

//     // deleting the members associated to the chat room group
//     if let Err(e) = diesel::delete(
//         chat_room_participants.filter(chat_room_participants::chat_room_id.eq(_chat_room_id)),
//     )
//     .execute(_conn)
//     {
//         return Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             format!("{:?}", e),
//         )));
//     }
//     // deleting the room from the chat rooms table
//     match diesel::delete(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
//         .execute(_conn)
//     {
//         Ok(_) => Ok(true),
//         Err(e) => {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!("{:?}", e),
//             )))
//         }
//     }
// }
