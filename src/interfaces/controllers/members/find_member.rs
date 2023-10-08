use axum::{
    http::{response::Parts, StatusCode},
    response::Response,
    Json,
};

use crate::{
    entities::member::MemberEntity,
    infrastructures::router::ApiResponse,
    interfaces::responses::{CustomError, NotFoundError},
    use_cases::members::find_member::{
        FindMemberInputData, FindMemberInteractor, FindMemberOutputData,
    },
};

// pub trait ResponseJson<T> {
//     fn res(code: StatusCode, body: T) -> (StatusCode, Json<T>);
//     fn response(res: (StatusCode, Json<T>)) -> (StatusCode, Json<T>);
//     fn response_self(self) -> (StatusCode, Json<T>);
// }

// impl<T, U> ResponseJson<T> for U
// where
//     T: std::fmt::Debug,
// {
//     fn res(code: StatusCode, body: T) -> (StatusCode, Json<T>) {
//         (code, Json(body))
//     }
//     fn response(res: (StatusCode, Json<T>)) -> (StatusCode, Json<T>) {
//         res
//     }
//     fn response_self(self) -> Self {
//         self
//     }
// }

pub struct FindMemberController {}

impl FindMemberController {
    pub async fn find_member(
        use_case: FindMemberInteractor,
        member_id: i32,
        // ) -> Result<
        //     // (StatusCode, Json<JsonResponse<MemberEntity>>),
        //     // (StatusCode, Json<JsonResponse<CustomError>>),
        //     (StatusCode, JsonResponse<MemberEntity>),
        //     (StatusCode, JsonRes1ponse<CustomError>),
        //     // impl ResponseJson<User> + ResponseJson<NotFoundError>
        // > {
    ) -> Result<(StatusCode, MemberEntity), (StatusCode, CustomError)> {
        let input_data = FindMemberInputData::new(member_id);
        let result = use_case.find_member(input_data).await;
        // let res = match result {
        //     Ok(result) => match result {
        //         None => Ok((
        //             StatusCode::NOT_FOUND,
        //             Json(JsonResponse::CustomError(CustomError::NotFoundError(
        //                 NotFoundError {
        //                     msg: "not found".to_string(),
        //                 },
        //             ))),
        //         )),
        //         Some(output) => Ok((
        //             StatusCode::OK,
        //             Json(JsonResponse::<MemberEntity>::OK(output.member)),
        //         )),
        //     },
        //     Err(err) => Err((
        //         StatusCode::INTERNAL_SERVER_ERROR,
        //         Json(JsonResponse::CustomError(err)),
        //     )),
        // };

        let res = match result {
            Ok(result) => match result {
                // None => Ok((
                //     StatusCode::NOT_FOUND,
                //     JsonResponse::CustomError(CustomError::NotFoundError(NotFoundError {
                //         msg: "not found".to_string(),
                //     })),
                // )),
                // ------------
                // None => Ok((
                //     StatusCode::NOT_FOUND,
                //     JsonResponse::CustomError(NotFoundError {
                //         msg: "not found".to_string(),
                //     }),
                // )),
                // Some(output) => Ok((StatusCode::OK, JsonResponse::<MemberEntity>::OK(output))),
                None => Err::<_, (StatusCode, CustomError)>((
                    StatusCode::NOT_FOUND,
                    CustomError::NotFoundError {
                        msg: "not found".to_string(),
                    },
                )),
                Some(output) => Ok::<(StatusCode, MemberEntity), _>((
                    StatusCode::OK,
                    // JsonResponse::<MemberEntity>::OK(output),
                    output,
                )),
            },
            Err(err) => {
                Err::<_, (StatusCode, CustomError)>((StatusCode::INTERNAL_SERVER_ERROR, err))
            }
        };
        res
    }
}
