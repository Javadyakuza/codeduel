use std::ops::Deref;
use std::str::FromStr;

use chrono::NaiveDateTime;

use rocket::{http::RawStr, *};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::db_models::Questions;

#[derive(FromForm, Debug, Serialize)]
pub struct EpQuQuestions {
    pub question_id: i32,          // 0 => None
    pub question_title: String,    // "" => None
    pub rival_id: i32,             // 0 => None
    pub question_category: String, // "" => All
}

#[derive(FromForm, Debug, Serialize)]
pub struct EpInQuestions {
    // pub question_id: i32, // pk
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub deadline: NaiveDateForm,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub category: String,
    pub reward: i32,
    pub entrance_fee: i32,
    pub test_inputs: String,
    pub test_outputs: String,
}
use crate::api_models::request::FromFormValue;

#[derive(Debug, Serialize)]
pub struct NaiveDateForm(NaiveDateTime);
impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        match NaiveDateTime::from_str(&form_value) {
            Ok(ndt) => Ok(NaiveDateForm(ndt)),
            Err(_) => Err(RawStr::from_str(
                "couldn't parse the deadline date and time from the string",
            )),
        }
        // here, parse NaiveDate and return an instance of the wrapper
    }
}
impl Deref for NaiveDateForm {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
