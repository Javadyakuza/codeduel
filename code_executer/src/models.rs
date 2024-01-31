use rocket::FromForm;
use serde::Serialize;

#[derive(FromForm, Debug, Serialize, Clone)]
pub struct CargoProjectParams {
    pub executable: String,
    pub executer: String,
}

