#![feature(decl_macro)] // helps us with the routing of our application

extern crate rocket; // imports all of the macros from the rocket crate

use codeduel_backend::api_models::*;
use codeduel_backend::db_models::*;
use codeduel_backend::wallet_lib::*;
use codeduel_backend::*;
use rocket::request::Form;
use rocket::request::Request;
use rocket::*;
use rocket_contrib::json::Json;
use solana_sdk::signature::Signature;

// #[get("/user-via-username/<username>")]
// fn get_user_via_username(username: String) -> Json<Result<QUsers, String>> {
//     let mut conn = establish_connection();
//     match get_user_with_username(&mut conn, username.as_str()) {
//         Ok(res) => return Json(Ok(res)),
//         Err(e) => return Json(Err(format!("{:?}", e))),
//     }
// }

#[post("/send-transaction", data = "<transaction_data>")]
fn send_transaction(transaction_data: Json<TransactionData>) -> Json<ApiResponse> {
    let wallet = Wallet::new("http://localhost:8899"); // Assuming a local Solana cluster
    let program_id = "YourProgramIdHere"; // Replace with your actual program ID

    match wallet.send_transaction_to_contract(program_id, &transaction_data.account_pubkey) {
        Ok(_) => Json(ApiResponse {
            success: true,
            message: "Transaction sent successfully".into(),
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            message: format!("Failed to send transaction: {}", e),
        }),
    }
}

#[post("/add-solana-wallet", data = "<new_wallet_info>")]
fn add_solana_wallet(new_wallet_info: Form<NewWalletIn>) -> Json<Result<QSolanaWallet, String>> {
    let mut conn = establish_connection();

    let _user_id;
    match get_user_with_username(&mut conn, &new_wallet_info.username_in) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }
    match initialize_new_solana_wallet(
        &mut conn,
        &SolanaWallet {
            user_id: _user_id,
            wallet_addr: new_wallet_info.wallet_addr_in.as_bytes().to_vec(),
            wallet_backup: new_wallet_info.wallet_backup_in.as_bytes().to_vec(),
        },
    ) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}

#[post("/create-token-account", data = "<new_wallet_info>")]
fn create_token_account_api(
    new_wallet_info: Form<CreateTokenAccount>,
) -> Json<Result<CreateTokenAccountResponse, String>> {
    match create_token_account(&CreateTokenAccount {
        wallet_address: new_wallet_info.wallet_address.clone(),
        token_mint_address: new_wallet_info.token_mint_address.clone(),
        token_program_id: new_wallet_info.token_program_id.clone(),
        lbh: new_wallet_info.lbh.clone(),
    }) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}
#[post("/fund-wallet", data = "<wallet_address>")]
fn fund_wallet(wallet_address: Form<FundWalletIn>) -> Json<Result<String, String>> {
    match activate_wallet_account_for_transfer(wallet_address.wallet_address.clone()) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}

#[get("/get-solana-addr-by-username/<username>")]
fn get_solana_addr(username: String) -> Json<Result<String, String>> {
    let mut conn = establish_connection();
    let _user_id;
    match get_user_with_username(&mut conn, &username) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    match get_user_solana_wallet(&mut conn, _user_id) {
        Ok(res) => Json(Ok(String::from_utf8_lossy(res.wallet_addr.as_slice())
            .as_ref()
            .to_string())),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no the {} path doesn't exists !!", req.uri())
}
fn main() {
    // rocket::ignite()
    //     .register(catchers![not_found])
    //     .mount("/api", routes![])
    //     // .attach(DbConn::fairing())
    //     .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
