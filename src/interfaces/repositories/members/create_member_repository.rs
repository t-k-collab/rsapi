use crate::{
    interfaces::repositories::members::MemberModel,
    use_cases::members::create_member::CreateMemberInputData,
};
use chrono::Utc;

pub struct CreateMemberRepository {}

// trait DataAccessInterface {
//     fn access_db(&self) -> Self;
// }

impl CreateMemberRepository {
    pub fn create(&self, input: CreateMemberInputData) -> MemberModel {
        println!("creating member data.");
        // TODO crate data on DB and fetch them.
        // self.access_db();

        // TODO implement Later
        let utc = Utc::now();
        MemberModel {
            member_id: 1,
            family_name: input.family_name,
            middle_name: input.middle_name,
            first_name: input.first_name,
            date_of_birth: utc.date_naive(),
            email: "member@email.com".to_string(),
            password: "password".to_string(),
            created_at: utc.naive_utc(),
            updated_at: utc.naive_utc(),
        }
    }
}
