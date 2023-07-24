use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateMemberInputData {
    pub family_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub pass_code: String,
}
