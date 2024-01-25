#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket; // imports all of the macros from the rocket crate

use codeduel_backend::api_models::{EpInQuestions, EpOutQuestions};
// use codeduel_backend::api_models::*;
use codeduel_backend::db_models::*;
// use codeduel_backend::wallet_lib::*;
use codeduel_backend::*;
use rocket::request::Form;
use rocket::request::Request;
use rocket::*;
use rocket_contrib::json::Json;
// use solana_sdk::signature::Signature;

// ------------- get endpoints ----------
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
    queryable_question: Form<EpInQuestions>,
) -> Json<Result<Vec<Questions>, String>> {
    let mut conn = establish_connection();

    match get_question(&mut conn, &QQuestions::build_from_ep(&*queryable_question)) {
        Ok(res) => return Json(Ok(res)),
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
        .mount("/api", routes![get_user_ep, get_question_ep])
        // .attach(DbConn::fairing())
        .launch();
}
