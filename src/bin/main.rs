#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket; // imports all of the macros from the rocket crate

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

#[post("/get_question", data = "<queryable_question>")]
fn get_question_ep(
    queryable_question: Form<EpQuQuestions>,
) -> Json<Result<Vec<Questions>, String>> {
    let mut conn = establish_connection();

    match get_question(&mut conn, &QQuestions::build_from_ep(&*queryable_question)) {
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
    let insertable_query_struct: IQuestions = IQuestions {
        rival_id: insertable_question.rival_id,
        question_title: insertable_question.question_title.clone(),
        question_body: insertable_question.question_body.clone(),
        deadline: *insertable_question.deadline.deref(),
        question_status: insertable_question.question_status,
        daredevil: insertable_question.daredevil,
        category: insertable_question.category.clone(),
        reward: insertable_question.reward,
        entrance_fee: insertable_question.entrance_fee,
    };
    match add_new_question(&mut conn, &insertable_query_struct, &mut tcs) {
        Ok(res) => return Json(Ok(res.0)),
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
            routes![get_user_ep, get_question_ep, add_user_ep, add_question_ep],
        )
        // .attach(DbConn::fairing())
        .launch();
}
