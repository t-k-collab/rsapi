use crate::use_cases::members::create_member::{
    CreateMemberInputData, CreateMemberInteractor, CreateMemberOutputData,
};

pub struct CreateMemberController {}

impl CreateMemberController {
    pub fn create_member(
        family_name: String,
        middle_name: Option<String>,
        last_name: String,
        pass_code: String,
    ) -> CreateMemberOutputData {
        // TODO implement correctly.
        // validated input_data
        let input_data = CreateMemberInputData {
            family_name,
            middle_name,
            last_name,
            pass_code,
        };

        // return output_data
        CreateMemberInteractor::create_member(input_data)
    }
}

// trait PrimitiveNum: Copy + Clone + Eq + Ord + PartialEq + PartialOrd {}
// impl<T: Copy + Clone + Eq + Ord + PartialEq + PartialOrd> PrimitiveNum for T {}
