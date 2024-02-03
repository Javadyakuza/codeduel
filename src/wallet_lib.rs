use crate::db_models::{QQuestions, Questions};
use crate::{get_question, get_user_wallet};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::Transaction;

pub use std::env;
use std::str::FromStr;

pub fn handle_solved_question(
    _conn: &mut PgConnection,
    _question: Questions,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // getting the winner sol address to transfer the reward  + entrance fee
    let daredevil_pubkey: Pubkey = match get_user_wallet(_conn, _question.daredevil.unwrap()) {
        Ok(w) => Pubkey::from_str(w.as_str()).unwrap(),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{}", e),
            )))
        }
    };

    // getting the rival account address to transfer the prize pool left sol tokens to,
    let rival_pubkey: Pubkey = match get_user_wallet(_conn, _question.rival_id) {
        Ok(w) => Pubkey::from_str(w.as_str()).unwrap(),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{}", e),
            )))
        }
    };

    // getting the platforms key pair (only for local tests)
    let kp = Keypair::read_from_file("/home/javadyakuza/rust_projects/codeduel_backend/sec.json")
        .unwrap();

    // getting the platforms accounts pub key
    let pk = kp.try_pubkey().unwrap();

    // setting the local vm jrpc endpoint
    let rpc = RpcClient::new("https://localhost::8899".to_string());

    // getting the latest block hash
    let lbh: Hash = match rpc.get_latest_blockhash() {
        Ok(_lbh) => _lbh,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("couldn't fetch the latest block hash due to \n {}", e),
            )))
        }
    };

    // calculating the dared evil share -1%
    let mut daredevil_share: u64 = ((_question.reward * 10_i32.pow(9)) as f64
        + (_question.entrance_fee as f64 * 10_i32.pow(9) as f64))
        as u64;

    daredevil_share = daredevil_share - daredevil_share / 100_u64;

    let rival_share: u64 = (_question.prize_pool * 10_i32.pow(9)) as u64 - daredevil_share;

    // sending the tokens to the daredevil
    match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
        &[transfer(&pk, &daredevil_pubkey, daredevil_share)],
        Some(&pk),
        &[&kp],
        Hash::from_str(lbh.to_string().as_str()).unwrap(),
    )) {
        Ok(sig) => {
            // funding the account
            match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
                &[transfer(&pk, &rival_pubkey, rival_share)],
                Some(&pk),
                &[&kp],
                Hash::from_str(lbh.to_string().as_str()).unwrap(),
            )) {
                Ok(sig2) => Ok(vec![sig.to_string(), sig2.to_string()]),
                Err(e2) => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{}", e2),
                ))),
            }
        }
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{}", e),
        ))),
    }
}

pub fn handle_dead_question(
    _conn: &mut PgConnection,
    _question_id: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let q = get_question(
        _conn,
        &QQuestions {
            question_id: Some(_question_id),
            question_title: None,
            rival_id: None,
            question_category: None,
        },
    )
    .unwrap(); // panic impossible

    // getting the rival account address to transfer the prize pool left sol tokens to,
    let rival_pubkey: Pubkey = match get_user_wallet(_conn, q[0].rival_id) {
        Ok(w) => Pubkey::from_str(w.as_str()).unwrap(),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{}", e),
            )))
        }
    };

    // getting the platforms key pair (only for local tests)
    let kp = Keypair::read_from_file("/home/javadyakuza/rust_projects/codeduel_backend/sec.json")
        .unwrap();

    // getting the platforms accounts pub key
    let pk = kp.try_pubkey().unwrap();

    // setting the local vm jrpc endpoint
    let rpc = RpcClient::new("https://localhost::8899".to_string());

    // getting the latest block hash
    let lbh: Hash = match rpc.get_latest_blockhash() {
        Ok(_lbh) => _lbh,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("couldn't fetch the latest block hash due to \n {}", e),
            )))
        }
    };

    // preparing the rival reward - 1% of the platform share
    let mut rival_share: u64 = (q[0].prize_pool * 10_i32.pow(9)) as u64;

    rival_share = rival_share - rival_share / 100_u64;

    // sending the tokens to the daredevil
    match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
        &[transfer(&pk, &rival_pubkey, rival_share)],
        Some(&pk),
        &[&kp],
        Hash::from_str(lbh.to_string().as_str()).unwrap(),
    )) {
        Ok(sig) => Ok(sig.to_string()),

        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{}", e),
        ))),
    }
}
