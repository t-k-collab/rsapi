use crate::use_cases::members::create_member::CreateMemberInputData;
use serde::Deserialize;

pub struct CreateMemberRepository {}

// trait DataAccessInterface {
//     fn access_db(&self) -> Self;
// }

impl CreateMemberRepository {
    pub fn create(&self, input: CreateMemberInputData) -> MemberModel {
        println!("creating member data.");
        // TODO crate data on DB and fetch them.
        // self.access_db();

        // TODO convert
        MemberModel {
            member_id: 1,
            family_name: input.family_name,
            middle_name: input.middle_name,
            first_name: input.first_name,
            pass_code: input.pass_code,
        }
    }
}

#[derive(Deserialize)]
pub struct MemberModel {
    pub member_id: u16,
    pub family_name: String,
    pub middle_name: String,
    pub first_name: String,
    pub pass_code: String,
    // email: Email,
    // created_at: ,
    // updated_at: ,
}
