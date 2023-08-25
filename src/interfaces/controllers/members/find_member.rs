use crate::use_cases::members::find_member::{
    FindMemberInputData, FindMemberInteractor, FindMemberOutputData,
};

pub struct FindMemberController {}

impl FindMemberController {
    pub async fn find_member(
        use_case: FindMemberInteractor,
        member_id: i32,
    ) -> FindMemberOutputData {
        let input_data = FindMemberInputData::new(member_id);
        use_case.find_member(input_data).await
    }
}
