use serde::{Deserialize, Serialize};

use crate::entities::member::MemberEntity;

// DTO<Input> validation should be here?
#[derive(Deserialize)]
pub struct CreateMemberInputData {
    pub family_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub pass_code: String,
    // pub email: String // TODO Value Object.
}

// validation trait, impl
// trait and impl CreateMemberInputData() {}

// DTO<Output>
#[derive(Debug, Serialize)]
pub struct CreateMemberOutputData {
    pub member: MemberEntity,
}

// Use Case implementation
pub struct CreateMemberInteractor {}

impl CreateMemberInteractor {
    pub fn create_member(input_data: CreateMemberInputData) -> CreateMemberOutputData {
        // TODO Dependency Inversion
        // TODO crate data on DB and fetch them.
        let fn_from_db = input_data.family_name;
        let ml_from_db = input_data.middle_name;
        let ln_from_db = input_data.last_name;
        let pc_from_db = input_data.pass_code;

        let member_entity = MemberEntity::new(fn_from_db, ml_from_db, ln_from_db, pc_from_db);

        CreateMemberOutputData {
            member: member_entity,
        }
    }
}
