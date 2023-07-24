use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberEntity {
    pub id: u16,
    pub family_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub pass_code: String,
    pub created: String, // TODO date type. time or chrono crate.
    pub updated: String, // TODO date type. time or chrono crate.
}

impl MemberEntity {
    pub fn new(
        family_name: String,
        middle_name: Option<String>,
        last_name: String,
        pass_code: String,
    ) -> MemberEntity {
        MemberEntity {
            id: 10, // TODO automatic
            family_name,
            middle_name: middle_name.unwrap_or_default(),
            last_name,
            pass_code,
            created: "20230720".to_string(),
            updated: "20230720".to_string(),
        }
    }
}
