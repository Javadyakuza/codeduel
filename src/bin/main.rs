#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket;
extern crate rocket_cors;
use codeduel_backend::api_models::{CargoProjectParams, EpInQuestions, EpQuQuestions};
use codeduel_backend::db_models::*;
use codeduel_backend::tc_execution_lib::{parse_init_execute, update_toml};
use codeduel_backend::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::Form;
use rocket::http::Header;
use rocket::request::Request;
use rocket::serde::json::Json;
use rocket::shield::Shield;
use rocket::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

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
async fn add_question_ep(
    insertable_question: Form<EpInQuestions>,
) -> Json<Result<Questions, String>> {
    let mut conn = establish_connection();
    let mut tcs: ITestCases = ITestCases {
        question_id: 0,
        executable_solution: insertable_question.executable_solution.clone(),
        solution_executer: insertable_question.solution_executer.clone(),
        test_inputs: insertable_question.test_inputs.clone(),
        test_outputs: insertable_question.test_inputs.clone(),
    };
    let insertable_query_struct: IQuestions;
    match IQuestions::from_ep_in_question(insertable_question.clone()) {
        Ok(q) => insertable_query_struct = q,
        Err(e) => return Json(Err(format!("{:?}", e))),
    }

    // testing the the passed samples of the question
    let temp_runner_params: CargoProjectParams = CargoProjectParams {
        executable: tcs.executable_solution.to_owned(),
        executer: tcs.solution_executer.to_owned(),
    };

    let test_cases_res: String = match parse_init_execute(temp_runner_params).await {
        Ok(res) => {
            if !res {
                return Json(Err("Running test cases failed".to_string()));
            }
            "true".to_string()
        }
        Err(e) => format!("{:?}", e),
    };

    let _ = rocket::tokio::time::sleep(rocket::tokio::time::Duration::from_secs(10)).await;

    match update_toml().await {
        Ok(_) => {}
        Err(e) => return Json(Err(format!("{:?}", e))),
    }

    if test_cases_res != "true".to_string() {
        return Json(Err(test_cases_res));
    }

    match add_new_question(&mut conn, &insertable_query_struct, &mut tcs).await {
        Ok(res) => return Json(Ok(res.0)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/try_solution", data = "<triable_solution>")]
async fn try_solution_ep(triable_solution: Form<IResponses>) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();

    let executer: String = match get_test_cases(&mut conn, triable_solution.question_id) {
        Ok(tcs) => tcs.solution_executer,
        Err(e) => format!("{:?}", e),
    };
    // testing the response
    let temp_runner_params: CargoProjectParams = CargoProjectParams {
        executable: triable_solution.response_code.to_owned(),
        executer: executer.to_owned(),
    };

    let test_cases_res: String = match parse_init_execute(temp_runner_params).await {
        Ok(res) => {
            if !res {
                return Json(Err("Running test cases failed".to_string()));
            }
            "true".to_string()
        }
        Err(e) => format!("{:?}", e),
    };

    let _ = rocket::tokio::time::sleep(rocket::tokio::time::Duration::from_secs(10)).await;

    match update_toml().await {
        Ok(_) => {}
        Err(e) => return Json(Err(format!("{:?}", e))),
    }

    if test_cases_res != "true".to_string() {
        return Json(Err(test_cases_res));
    }

    match add_response(&mut conn, &triable_solution) {
        Ok(_) => return Json(Ok(true)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

// #[post("/add_response", data = "<insertable_response>")]
// fn add_response_ep(insertable_response: Form<IResponses>) -> Json<Result<Responses, String>> {
//     let mut conn = establish_connection();

//     match add_response(&mut conn, &insertable_response) {
//         Ok(res) => return Json(Ok(res)),
//         Err(e) => return Json(Err(format!("{:?}", e))),
//     }
// }

#[post("/add_user_wallet", data = "<insertable_user_wallet>")]
fn add_user_wallet_ep(insertable_user_wallet: Form<Wallets>) -> Json<Result<String, String>> {
    let mut conn = establish_connection();

    match add_user_wallet(&mut conn, &insertable_user_wallet) {
        Ok(res) => return Json(Ok(res.sol_addr)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

// ------------- update endpoints ---------- //

#[put("/update_user", data = "<updatable_user>")]
fn update_user_ep(updatable_user: Form<UUser>) -> Json<Result<Users, String>> {
    let mut conn = establish_connection();

    match update_user(&mut conn, &updatable_user) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}
#[put("/update_user_wallet", data = "<updatable_user_wallet>")]
fn update_user_wallet_ep(updatable_user_wallet: Form<UWallets>) -> Json<Result<Wallets, String>> {
    let mut conn = establish_connection();

    match update_user_wallet(&mut conn, &updatable_user_wallet) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[put("/update_question", data = "<updatable_question>")]
fn update_question_ep(updatable_question: Form<UQuestion>) -> Json<Result<Questions, String>> {
    let mut conn = establish_connection();

    match update_question(&mut conn, &updatable_question) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

// ------------- update endpoints ---------- //
#[delete("/delete_user", data = "<removable_user>")]
fn delete_user_ep(removable_user: Form<RUsers>) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();

    match delete_user(&mut conn, &removable_user) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[delete("/delete_question", data = "<removable_question>")]
fn delete_question_ep(removable_question: Form<RQuestions>) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();

    match delete_question(&mut conn, &removable_question) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no, we don't know where is {} ", req.uri())
}
pub struct Cors;
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    // Allowed origins can be specified as exact strings or as regex patterns
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8000", // Specify your frontend origin here
    ]);

    // Configure CORS
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            rocket_http::Method::Get,
            rocket_http::Method::Post,
            rocket_http::Method::Put,
            rocket_http::Method::Delete,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let _ = rocket::build()
        .attach(Shield::new())
        .attach(cors)
        .register("/", catchers![not_found])
        .mount(
            "/api",
            routes![
                get_user_ep,
                get_question_ep,
                add_user_ep,
                add_question_ep,
                try_solution_ep,
                add_user_wallet_ep,
                update_user_ep,
                update_user_wallet_ep,
                update_question_ep,
                delete_user_ep,
                delete_question_ep,
            ],
        )
        .launch()
        .await;

    Ok(())
}
