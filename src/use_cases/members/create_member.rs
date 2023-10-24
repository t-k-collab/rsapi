use chrono::NaiveDate;
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
    pub date_of_birth: Option<NaiveDate>,
    pub email: String,
    pub password: String,
    // pub email: Email // TODO Value Object.
}

// validation, initializing
impl CreateMemberInputData {
    pub fn new(
        family_name: String,
        middle_name: Option<String>,
        first_name: String,
        date_of_birth: Option<NaiveDate>,
        email: String,
        password: String,
    ) -> Self {
        Self {
            family_name,
            middle_name: middle_name.unwrap_or_default(), // Application Logic.
            first_name,
            password,
            date_of_birth,
            email,
        }
    }
}

// DTO<Output>

pub type CreateMemberOutputData = MemberEntity;

pub struct CreateMemberInteractor {
    pub repo: CreateMemberRepository,
}

impl CreateMemberInteractor {
    pub async fn create_member(&self, input_data: CreateMemberInputData) -> CreateMemberOutputData {
        // pub async fn create_member(&self, input_data: CreateMemberInputData) ->   Result<Option<CreateMemberOutputData>, CustomError>{
        // Dependency Inversion
        let model = self.repo.create(input_data).await;

        // TODO convert model to entity;
        let member = MemberEntity::new(
            model.member_id,
            model.family_name,
            model.middle_name,
            model.first_name,
            model.date_of_birth,
            model.email,
            vec![],
        );

        member
    }
}
