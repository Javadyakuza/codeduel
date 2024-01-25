use std::fmt::Display;

use chrono::NaiveDateTime;
use rocket::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::db_models::Questions;

#[derive(FromForm, Debug, Serialize)]
pub struct EpInQuestions {
    pub question_id: i32,          // 0 => None
    pub question_title: String,    // "" => None
    pub rival_id: i32,             // 0 => None
    pub question_category: String, // "" => All
}

#[derive(Debug, Serialize)]
pub struct EpOutQuestions {
    pub question_id: i32,
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub creation_time: Ndt,
    pub deadline: Ndt,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub reward: i32,
    pub prize_pool: i32,
    pub entrance_fee: i32,
    pub category: String,
}
#[derive(Debug)]
pub struct Ndt(NaiveDateTime);

impl Serialize for Ndt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct("ndt", self.0.to_string().as_str())
    }
}

impl EpOutQuestions {
    pub fn from_questions(q: Vec<Questions>) -> Vec<EpOutQuestions> {
        q.iter()
            .map(|x| EpOutQuestions {
                question_id: x.question_id,
                rival_id: x.rival_id,
                question_title: x.question_title.clone(),
                question_body: x.question_body.clone(),
                creation_time: Ndt(x.creation_time),
                deadline: Ndt(x.deadline),
                question_status: x.question_status,
                daredevil: x.daredevil,
                reward: x.reward,
                prize_pool: x.prize_pool,
                entrance_fee: x.entrance_fee,
                category: x.category.clone(),
            })
            .collect()
    }
}
