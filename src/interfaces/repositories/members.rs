use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::FromRow;

pub mod create_member_repository;
pub mod find_member_repository;

#[derive(FromRow, Debug, Serialize)]
pub struct MemberModel {
    pub member_id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub family_name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(FromRow, Debug, Serialize)]
pub struct MemberWithFamilyInfoModel {
    pub member_id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub family_name: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub family_id: i32,
    pub name: String,
}
