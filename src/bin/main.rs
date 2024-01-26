#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket; // imports all of the macros from the rocket crate

use std::borrow::Borrow;
use std::ops::Deref;

use codeduel_backend::api_models::EpInQuestions;
use codeduel_backend::api_models::EpQuQuestions;
// use codeduel_backend::api_models::*;
use codeduel_backend::db_models::*;
// use codeduel_backend::wallet_lib::*;
use codeduel_backend::*;
use rocket::request::Form;
use rocket::request::Request;
use rocket::*;
use rocket_contrib::json::Json;
// use solana_sdk::signature::Signature;

// ------------- get endpoints ---------- //
#[get("/get_user/<username_or_id>")]
fn get_user_ep(username_or_id: String) -> Json<Result<Users, String>> {
    let mut conn = establish_connection();
    match get_user(&mut conn, username_or_id.as_str()) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/get_question/<_question_id>/<_question_title>/<_rival_id>/<_question_category>")]
fn get_question_ep(
    _question_id: i32,
    _question_title: String,
    _rival_id: i32,
    _question_category: String,
) -> Json<Result<Vec<Questions>, String>> {
    let mut conn = establish_connection();

    match get_question(
        &mut conn,
        &QQuestions::build_from_ep(&EpQuQuestions {
            question_id: _question_id,
            question_title: _question_title,
            rival_id: _rival_id,
            question_category: _question_category,
        }),
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

// ------------- post endpoints ---------- //
#[post("/add_user", data = "<insertable_user>")]
fn add_user_ep(insertable_user: Form<IUsers>) -> Json<Result<Users, String>> {
    let mut conn = establish_connection();

    match add_new_user(&mut conn, &insertable_user) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/add_question", data = "<insertable_question>")]
fn add_question_ep(insertable_question: Form<EpInQuestions>) -> Json<Result<Questions, String>> {
    let mut conn = establish_connection();
    let mut tcs: ITestCases = ITestCases {
        question_id: 0,
        test_inputs: insertable_question.test_inputs.clone(),
        test_outputs: insertable_question.test_inputs.clone(),
    };
    let insertable_query_struct: IQuestions;
    match IQuestions::from_ep_in_question(insertable_question.clone()) {
        Ok(q) => insertable_query_struct = q,
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
    match add_new_question(&mut conn, &insertable_query_struct, &mut tcs) {
        Ok(res) => return Json(Ok(res.0)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/add_response", data = "<insertable_response>")]
fn add_response_ep(insertable_response: Form<IResponses>) -> Json<Result<Responses, String>> {
    let mut conn = establish_connection();

    match add_response(&mut conn, &insertable_response) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/add_user_wallet", data = "<insertable_user_wallet>")]
fn add_user_wallet_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

// ------------- update endpoints ---------- //

#[put("/update_user", data = "<insertable_user_wallet>")]
fn update_user_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}
#[put("/update_user_wallet", data = "<insertable_user_wallet>")]
fn update_user_wallet_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[put("/update_question", data = "<insertable_user_wallet>")]
fn update_question_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

// ------------- update endpoints ---------- //
#[delete("/delete_user", data = "<insertable_user_wallet>")]
fn delete_user_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[delete("/delete_question", data = "<insertable_user_wallet>")]
fn delete_question_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no, we don't know where is {} ", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/api",
            routes![
                get_user_ep,
                get_question_ep,
                add_user_ep,
                add_question_ep,
                add_response_ep,
                add_user_wallet_ep,
                update_user_ep,
                update_user_wallet_ep,
                update_question_ep,
                delete_question_ep,
                delete_user_ep,
            ],
        )
        // .attach(DbConn::fairing())
        .launch();
}
