use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterClassCreatePayload {
    pub title: String,
    pub description: String,
    pub timing: String,
    pub date: NaiveDate,
    pub presenter_name: String,
    pub admin_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterClassEnrollPayload {
    pub master_class_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub contact: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterClassEnquirePayload {
    pub master_class_id: i64,
    pub contact: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct MasterClassRecord {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub timing: String,
    pub date: NaiveDate,
    pub presenter_name: String,
}
