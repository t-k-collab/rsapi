use serde::{Deserialize, Serialize};

use crate::{
    entities::member::MemberEntity,
    interfaces::repositories::members::create_member_repository::CreateMemberRepository,
};

// DTO<Input> validation should be here?
#[derive(Deserialize)]
pub struct CreateMemberInputData {
    pub family_name: String,
    pub middle_name: String,
    pub first_name: String,
    pub pass_code: String,
    // pub email: Email // TODO Value Object.
}

// validation, initializing
impl CreateMemberInputData {
    pub fn new(
        family_name: String,
        middle_name: Option<String>,
        first_name: String,
        pass_code: String,
    ) -> Self {
        Self {
            family_name,
            middle_name: middle_name.unwrap_or_default(), // Application Logic.
            first_name,
            pass_code,
        }
    }
}

// DTO<Output>
#[derive(Debug, Serialize)]
pub struct CreateMemberOutputData {
    pub member: MemberEntity,
}

pub struct CreateMemberInteractor {
    pub repo: CreateMemberRepository,
}

impl CreateMemberInteractor {
    pub fn create_member(&self, input_data: CreateMemberInputData) -> CreateMemberOutputData {
        // Dependency Inversion
        let model = self.repo.create(input_data);

        // TODO convert model to entity;
        let member = MemberEntity::new(
            model.member_id,
            model.family_name,
            model.middle_name,
            model.first_name,
            model.date_of_birth,
            model.email,
        );

        CreateMemberOutputData { member }
    }
}
