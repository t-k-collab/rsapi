use serde::{Deserialize, Serialize};

use crate::{
    entities::member::MemberEntity,
    interfaces::{
        repositories::members::find_member_repository::FindMemberRepository, responses::CustomError,
    },
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
// #[derive(Debug, Serialize)]
// pub struct FindMemberOutputData {
//     pub member: MemberEntity,
// }

pub type FindMemberOutputData = MemberEntity;

pub struct FindMemberInteractor {
    pub repo: FindMemberRepository,
}

impl FindMemberInteractor {
    pub async fn find_member(
        &self,
        input_data: FindMemberInputData,
    ) -> Result<Option<FindMemberOutputData>, CustomError> {
        // Dependency Inversion
        // let result = self.repo.find(input_data).await;
        let result = self.repo.find_with_family_info(input_data).await;

        match result {
            Ok(None) => return Ok(None), // here will not be passed?? If there are no members, it will return empty array [].
            Ok(Some(res)) => {
                let family_ids = if res.len() == 0 {
                    return Ok(None);
                } else {
                    res.iter()
                        .map(|mfi| (mfi.family_id, mfi.name.to_string()))
                        .collect()
                };

                let member = MemberEntity::new(
                    res[0].member_id,
                    res[0].family_name.to_string(),
                    res[0].middle_name.to_string(),
                    res[0].first_name.to_string(),
                    res[0].date_of_birth,
                    res[0].email.to_string(),
                    family_ids,
                );

                Ok(Some(member))
            }
            Err(e) => Err(e),
        }
    }
}
