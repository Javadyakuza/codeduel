#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket; // imports all of the macros from the rocket crate

// use codeduel_backend::api_models::*;
use codeduel_backend::db_models::*;
// use codeduel_backend::wallet_lib::*;
use codeduel_backend::*;
use rocket::request::Form;
use rocket::request::Request;
use rocket::*;
use rocket_contrib::json::Json;
// use solana_sdk::signature::Signature;

// ------------- get gateways ----------
#[get("/get_user/<username_or_id>")]
fn get_user_ep(username_or_id: String) -> Json<Result<Users, String>> {
    let mut conn = establish_connection();
    match get_user(&mut conn, username_or_id.as_str()) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no , we don't know where is {} url ", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/api", routes![get_user_ep])
        // .attach(DbConn::fairing())
        .launch();
}
