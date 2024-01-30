use rocket::*;

use serde::Serialize;

#[derive(FromForm, Debug, Serialize)]
pub struct EpQuQuestions {
    pub question_id: i32,          // 0 => None
    pub question_title: String,    // "" => None
    pub rival_id: i32,             // 0 => None
    pub question_category: String, // "" => All
}

#[derive(FromForm, Debug, Serialize, Clone)]
pub struct EpInQuestions {
    // pub question_id: i32, // pk
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub deadline: String,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub category: String,
    pub reward: i32,
    pub entrance_fee: f32,
    pub executable_solution: String,
    pub solution_executer: String,
    pub test_inputs: String,
    pub test_outputs: String,
}
