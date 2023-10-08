use axum::{
    extract::{Path, State},
    http::{Error, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};

use crate::{
    entities::member::MemberEntity,
    interfaces::{
        controllers::members::{
            create_member::CreateMemberController, find_member::FindMemberController,
        },
        repositories::members::find_member_repository::FindMemberRepository,
        responses::CustomError,
    },
    use_cases::members::{
        create_member::CreateMemberInputData,
        find_member::{FindMemberInteractor, FindMemberOutputData},
    },
};

pub fn init_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/members/:member_id", get(get_member))
        .route("/members", post(create_member))
        .with_state(pool)
}

pub type ApiResponse<T> = (StatusCode, Json<T>);
pub type ApiErrResponse = (StatusCode, Json<CustomError>);

// pub type TApiResponse<T> = (StatusCode, Json<JsonResponse<T>>);
// pub type TApiErrResponse = (StatusCode, Json<JsonResponse<CustomError>>);
// #[derive(Debug)]
// pub enum APIResponse<T> {
//     Normal((StatusCode, Json<JsonResponse<T>>)),
//     Error((StatusCode, Json<JsonResponse<CustomError>>)),
// }

// REFACTOR move this and refactor later.
#[derive(Debug, Deserialize, Serialize)]
struct SampleCustomError {
    msg: String,
}

// REFACTOR move this and refactor later.
#[derive(Debug, Deserialize, Serialize)]
struct Res<T> {
    res_body: Option<T>,
    is_err: bool,
    err: Option<SampleCustomError>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Body_Test {
    str: String,
}

// TODO Change later
async fn health_check(State(pool): State<PgPool>) -> impl IntoResponse {
    {
        let row = sqlx::query!("SELECT * FROM members").fetch_all(&pool).await;

        let res = if row.is_err() {
            let body = Res {
                res_body: None,
                is_err: true,
                err: Some(SampleCustomError {
                    msg: "error".to_string(),
                }),
            };
            let err_res: ApiResponse<Res<Body_Test>> = (
                StatusCode::INTERNAL_SERVER_ERROR, // StatusCode::NOT_FOUND,
                Json(body),
            );
            err_res
        } else {
            let result = match row.ok() {
                Some(res) => {
                    println!("There is something.");
                    if res.is_empty() {
                        let body = Res {
                            res_body: None,
                            is_err: false,
                            err: None,
                        };
                        return (StatusCode::OK, Json(body));
                    }
                    let sample = &res[0]; // if there is a recode, get the first one.
                    let a = &sample.family_name;

                    let body = Res {
                        res_body: Some(Body_Test { str: a.to_string() }),
                        is_err: false,
                        err: None,
                    };

                    let err_res: ApiResponse<Res<Body_Test>> = (StatusCode::OK, Json(body));
                    err_res
                }
                None => {
                    let body = Res {
                        res_body: None,
                        is_err: true,
                        err: Some(SampleCustomError {
                            msg: "error".to_string(),
                        }),
                    };
                    let err_res: ApiResponse<Res<Body_Test>> = (StatusCode::NOT_FOUND, Json(body));
                    err_res
                }
            };
            result
        };
        res
    }
}

async fn get_member(
    State(pool): State<PgPool>,
    Path(member_id): Path<i32>,
    // ) -> Result<impl IntoResponse, impl IntoResponse> {
    // ) -> impl ResponseJson<MemberEntity> + ResponseJson<CustomError> {
) -> impl IntoResponse {
    {
        println!("path param member_id: {}", member_id);

        let repo = FindMemberRepository { pool };
        let use_case = FindMemberInteractor { repo };

        let output_data = FindMemberController::find_member(use_case, member_id).await;

        match output_data {
            Ok(res) => (res.0, Json(res.1)).into_response(),
            Err(e) => (e.0, Json(e.1)).into_response(),
        }

        // match a {
        //     Ok(res) => res.into_response(),
        //     Err(e) => e.into_response(),
        // }

        // before
        // match output_data {
        //     // Ok(n) => Ok(n),
        //     // Err(e) => Err(e),
        //     // Ok(n) => APIResponse::Normal(n),
        //     // Err(e) => APIResponse::Error(e),
        //     Ok(n) => {
        //         let stc = n.0;
        //         let s = n.1;
        //         match s {
        //             // JsonResponse::OK(member) => (stc, Json(member)).into_response(),
        //             // JsonResponse::CustomError(e) => {
        //             //     let t = CustomError::NotFoundError(e);
        //             //     (stc, Json(t)).into_response()
        //             // }
        //             JsonResponse::OK(member) => Ok((stc, Json(member)).into_response()),
        //             JsonResponse::CustomError(e) => {
        //                 match e {
        //                     CustomError::NotFoundError { msg } => {
        //                         let t = SampleCustomError { msg };
        //                         Ok((stc, Json(t)).into_response())
        //                     }
        //                     CustomError::InternalError { msg } => {
        //                         let t = SampleCustomError { msg };
        //                         Ok((stc, Json(t)).into_response())
        //                     }
        //                     _ => Err(e.into_response()),
        //                 }
        //                 // let t = e;
        //                 // Ok((stc, Json(t)).into_response())
        //             }
        //         }
        //         // (stc, Json(json))
        //     }
        //     Err(e) => Err(e.into_response()),
        // }
        // // println!("res: {:#?}", res);
        // // res.into_response()
        // // before until here.
    }
}

async fn create_member(
    State(_pool): State<PgPool>,
    Json(payload): Json<CreateMemberInputData>,
) -> impl IntoResponse {
    println!(
        "payload: {}, {:?}, {}, {}",
        payload.family_name,
        Some(&payload.middle_name),
        payload.first_name,
        payload.pass_code
    );

    let output_data = CreateMemberController::create_member(
        payload.family_name,
        Some(payload.middle_name),
        payload.first_name,
        payload.pass_code,
    )
    .member;

    let member: ApiResponse<MemberEntity> = (StatusCode::CREATED, Json(output_data));
    member
}

// remove here.
#[cfg(test)]
mod tests {
    // use super::{CreateMemberInputData, MemberEntity};

    #[test]
    fn test_sample() {
        let expected = 4;
        assert_eq!(2 + 2, expected);
    }
}
