use serde::Deserialize;

use crate::{
    entities::member::{FamilyEntity, MemberEntity},
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
            Ok(None) => {
                println!("get response None from repository find_with_family_info.");
                Ok(None)
            } // here will not be passed?? If there are no members, it will return empty array [].
            Ok(Some(res)) => {
                let families = if res.is_empty() {
                    return Ok(None);
                } else {
                    res.iter()
                        .map(|mfi| FamilyEntity {
                            id: mfi.family_id,
                            name: mfi.name.to_string(),
                        })
                        .collect()
                };

                // res[index_no] values must be same.
                let member = MemberEntity::new(
                    res[0].member_id,
                    res[0].family_name.to_string(),
                    res[0].middle_name.to_string(),
                    res[0].first_name.to_string(),
                    res[0].date_of_birth,
                    res[0].email.to_string(),
                    families,
                );

                Ok(Some(member))
            }
            Err(e) => Err(e),
        }
    }
}
