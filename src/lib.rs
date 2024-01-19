#![recursion_limit = "256"]
pub mod api_models;
pub mod db_models;
pub mod schema;
use crate::db_models::{
    IQuestions, IResponses, ITestCases, IUsers, Questions, Responses, TestCases, Users, Wallets,
};
use crate::schema::{questions, responses, test_cases, users, wallets};
use chrono::Local;
use db_models::QResponses;
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
// unchecked - un-optimized
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
    _is_id: bool,
) -> Result<Users, Box<dyn std::error::Error>> {
    // fetching the user id if the user name was provided
    if !_is_id {
        let tmp_users: Vec<Users> = users
            .filter(users::username.eq(_username_or_id))
            .select(Users::as_select())
            .load(_conn)
            .unwrap_or(vec![]);
        if tmp_users.len() == 0 {
            // chat room id doesn't exists
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "username not found !",
            )))
        } else {
            Ok(tmp_users[0].to_owned())
        }
    } else {
        let _user_id: i32;
        match _username_or_id.parse::<i32>() {
            Ok(ui) => _user_id = ui,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid user id string",
                )))
            }
        }
        let tmp_users: Vec<Users> = users
            .filter(users::username.eq(_username_or_id))
            .select(Users::as_select())
            .load(_conn)
            .unwrap_or(vec![]);
        if tmp_users.len() == 0 {
            // chat room id doesn't exists
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "username not found !",
            )))
        } else {
            Ok(tmp_users[0].to_owned())
        }
    }
}

pub fn get_response(
    _conn: &mut PgConnection,
    _query_struct: &QResponses,
) -> Result<Responses, Box<dyn std::error::Error>> {
    // checking the format of the struct
    if (_query_struct.question_id.is_none()
        && (!_query_struct.response_id.is_some() | !_query_struct.daredevil_id.is_some()))
        | (_query_struct.question_id.is_some()
            && (!_query_struct.response_id.is_none() | !_query_struct.daredevil_id.is_none()))
    {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "the response query format is wrong ! \n
            supported formats: \n
            1 - Some(question_id), None(response_id), None(daredevil_id) \n
            2 - None(question_id), Some(response_id), Some(daredevil_id)",
        )));
    } else {
        //    fetching the user id if the user name was provided
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

// // general ** updaters
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

// // general removers
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
// pub fn get_chat_room_participants_by_id(
//     _conn: &mut PgConnection,
//     _chat_room_id: i32,
// ) -> Result<Vec<ChatRoomParticipants>, Box<dyn std::error::Error>> {
//     // getting the participants
//     let participants: Vec<ChatRoomParticipants> = chat_room_participants
//         .filter(chat_room_participants::chat_room_id.eq(_chat_room_id))
//         .select(ChatRoomParticipants::as_select())
//         .load(_conn)
//         .unwrap_or(vec![]);

//     if participants.len() == 0 {
//         // chat room id doesn't exists
//         Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::NotFound,
//             "chat room id not found or it has no members!",
//         )))
//     } else {
//         Ok(participants)
//     }
// }

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
