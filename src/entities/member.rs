use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberEntity {
    pub member_id: i32,
    pub family_name: String,
    pub middle_name: String,
    pub first_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub email: String, // TODO value object
    pub families: Vec<FamilyEntity>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FamilyEntity {
    pub id: i32,
    pub name: String,
}

impl MemberEntity {
    pub fn new(
        member_id: i32,
        family_name: String,
        middle_name: String,
        first_name: String,
        date_of_birth: Option<NaiveDate>,
        email: String,
        families: Vec<FamilyEntity>,
    ) -> Self {
        Self {
            member_id,
            family_name,
            middle_name,
            first_name,
            date_of_birth,
            email,
            families,
        }
    }
}
