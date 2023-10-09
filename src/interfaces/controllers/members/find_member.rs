use axum::http::StatusCode;

use crate::{
    interfaces::responses::CustomError,
    use_cases::members::find_member::{
        FindMemberInputData, FindMemberInteractor, FindMemberOutputData,
    },
};

pub struct FindMemberController {}

impl FindMemberController {
    pub async fn find_member(
        use_case: FindMemberInteractor,
        member_id: i32,
    ) -> Result<(StatusCode, FindMemberOutputData), (StatusCode, CustomError)> {
        let input_data = FindMemberInputData::new(member_id);
        let result = use_case.find_member(input_data).await;

        match result {
            Ok(result) => match result {
                None => Err::<_, (StatusCode, CustomError)>((
                    StatusCode::NOT_FOUND,
                    CustomError::NotFoundError {
                        msg: "not found".to_string(),
                    },
                )),
                Some(output) => {
                    Ok::<(StatusCode, FindMemberOutputData), _>((StatusCode::OK, output))
                }
            },
            Err(err) => {
                Err::<_, (StatusCode, CustomError)>((StatusCode::INTERNAL_SERVER_ERROR, err))
            }
        }
    }
}
