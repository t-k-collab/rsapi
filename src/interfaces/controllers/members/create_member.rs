use chrono::NaiveDate;

use crate::{
    interfaces::repositories::members::create_member_repository::CreateMemberRepository,
    use_cases::members::create_member::{
        CreateMemberInputData, CreateMemberInteractor, CreateMemberOutputData,
    },
};

pub struct CreateMemberController {}

impl CreateMemberController {
    pub async fn create_member(
        use_case: CreateMemberInteractor,
        family_name: String,
        middle_name: Option<String>,
        first_name: String,
        date_of_birth: Option<NaiveDate>,
        email: String,
        password: String,
    ) -> CreateMemberOutputData {
        let input_data = CreateMemberInputData::new(
            family_name,
            middle_name,
            first_name,
            date_of_birth,
            email,
            password,
        );

        // return output_data
        use_case.create_member(input_data).await
    }
}
