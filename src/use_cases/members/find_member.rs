use serde::{Deserialize, Serialize};

use crate::{
    entities::member::MemberEntity,
    interfaces::repositories::members::find_member_repository::FindMemberRepository,
};

// DTO<Input>
#[derive(Deserialize)]
pub struct FindMemberInputData {
    pub member_id: i32,
}

// validation, initializing
impl FindMemberInputData {
    pub fn new(member_id: i32) -> Self {
        // application validation
        //  TODO member_id must be more than 0.
        Self { member_id }
    }
}

// DTO<Output>
#[derive(Debug, Serialize)]
pub struct FindMemberOutputData {
    pub member: MemberEntity,
}

pub struct FindMemberInteractor {
    pub repo: FindMemberRepository,
}

impl FindMemberInteractor {
    pub async fn find_member(&self, input_data: FindMemberInputData) -> FindMemberOutputData {
        // Dependency Inversion
        let model = self.repo.find(input_data).await;

        let member = MemberEntity::new(
            model.member_id,
            model.family_name,
            model.middle_name,
            model.first_name,
            model.date_of_birth,
            model.email,
        );

        FindMemberOutputData { member }
    }
}
