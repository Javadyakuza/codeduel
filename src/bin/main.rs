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

// #[get("/user-via-username/<username>")]
// fn get_user_via_username(username: String) -> Json<Result<QUsers, String>> {
//     let mut conn = establish_connection();
//     match get_user_with_username(&mut conn, username.as_str()) {
//         Ok(res) => return Json(Ok(res)),
//         Err(e) => return Json(Err(format!("{:?}", e))),
//     }
// }
fn main() {
    // rocket::ignite()
    //     .register(catchers![not_found])
    //     .mount("/api", routes![])
    //     // .attach(DbConn::fairing())
    //     .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
