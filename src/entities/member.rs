use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MemberEntity {
    pub member_id: i32,
    pub family_name: String,
    pub middle_name: String,
    pub first_name: String,
    pub date_of_birth: String,
    pub email: String,
}

impl MemberEntity {
    pub fn new(
        member_id: i32,
        family_name: String,
        middle_name: String,
        first_name: String,
        date_of_birth: NaiveDate,
        email: String,
    ) -> Self {
        Self {
            member_id,
            family_name,
            middle_name,
            first_name,
            date_of_birth: date_of_birth.to_string(),
            email,
        }
    }
}
