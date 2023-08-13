use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberEntity {
    pub member_id: u16,
    pub family_name: String,
    pub middle_name: String,
    pub first_name: String,
    pub pass_code: String,
    pub created: String, // TODO date type. time or chrono crate. // TODO this is only for model.
    pub updated: String, // TODO date type. time or chrono crate. // TODO this is only for model.
}

impl MemberEntity {
    pub fn new(
        member_id: u16,
        family_name: String,
        middle_name: String,
        first_name: String,
        pass_code: String,
    ) -> Self {
        Self {
            member_id,
            family_name,
            middle_name,
            first_name,
            pass_code,
            created: "20230720".to_string(), // TODO fill
            updated: "20230720".to_string(), // TODO fill
        }
    }
}
