use crate::{
    interfaces::repositories::members::create_member_repository::CreateMemberRepository,
    use_cases::members::create_member::{
        CreateMemberInputData, CreateMemberInteractor, CreateMemberOutputData,
    },
};

pub struct CreateMemberController {}

impl CreateMemberController {
    pub fn create_member(
        family_name: String,
        middle_name: Option<String>,
        first_name: String,
        pass_code: String,
    ) -> CreateMemberOutputData {
        let input_data =
            CreateMemberInputData::new(family_name, middle_name, first_name, pass_code);

        let repo = CreateMemberRepository {};
        let use_case = CreateMemberInteractor { repo };

        // return output_data
        use_case.create_member(input_data)
    }
}
