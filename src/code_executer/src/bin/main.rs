#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket;
use code_executer::parse_init_execute;

use code_executer::update_toml;
use code_executer::CargoProjectParams;

use rocket::*;

#[post("/write_file", data = "<data>")]
async fn write_file(data: String) -> String {
    let parsed_str = data.split("---------------").collect::<Vec<&str>>();
    let parsed_data: CargoProjectParams = CargoProjectParams {
        executable: parsed_str[0].to_string(),
        executer: parsed_str[1].to_string(),
    };

    let out_put: String = match parse_init_execute(parsed_data).await {
        Ok(res) => res.to_string(),
        Err(e) => e.to_string(),
    };

    let _ = rocket::tokio::time::sleep(rocket::tokio::time::Duration::from_secs(10)).await;

    match update_toml().await {
        Ok(_) => {
            return out_put;
        }
        Err(e) => {
            println!("{}", e);
            return e.to_string();
        }
    }
}

// #[catch(404)]
// fn not_found(req: &Request) -> String {
//     format!("Oh no, we don't know where is {} ", req.uri())
// }
#[launch]
fn rocket() -> _ {
    rocket::build()
        // .register(catchers![not_found])
        .mount("/api", routes![write_file,])
    // .attach(DbConn::fairing())
    // .launch();
}
