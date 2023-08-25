use std::mem;

use axum::{
    body::{self, Body, BoxBody},
    extract::{Path, State},
    http::{Error, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::types, FromRow, PgPool, Pool, Postgres, Type};

use crate::{
    entities::member::MemberEntity,
    interfaces::{
        controllers::members::{
            create_member::CreateMemberController, find_member::FindMemberController,
        },
        repositories::members::find_member_repository::FindMemberRepository,
    },
    use_cases::members::{create_member::CreateMemberInputData, find_member::FindMemberInteractor},
};

use chrono::{NaiveDate, NaiveDateTime, Utc};

pub fn init_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        // .route("/", get(health_check))
        .route("/members/:member_id", get(get_member))
        .route("/members", post(create_member))
        .with_state(pool)
}

type ApiResponse<T> = (StatusCode, Json<T>);

// async fn health_check(State(pool): State<PgPool>) {
//     {
//         let row = sqlx::query!("SELECT * FROM members").fetch_all(&pool).await;

//         let result = match row {
//             Ok(res) => (StatusCode::OK, Json(res)),
//             Err(msg) => panic!("{:?}", msg),
//         };
//         let sample: &Json<Vec<sqlx::postgres::PgRow>> = &result.1;
//         println!("{:#?}", sample);
//     }
// }

async fn get_member(
    State(pool): State<PgPool>,
    Path(member_id): Path<i32>,
    // Path(first_name): Path<String>,
) -> impl IntoResponse {
    {
        println!("path param member_id: {}", member_id);

        let repo = FindMemberRepository { pool };
        let use_case = FindMemberInteractor { repo };

        let output_data = FindMemberController::find_member(use_case, member_id)
            .await
            .member;

        let res: ApiResponse<MemberEntity> = (StatusCode::OK, Json(output_data));
        res.into_response()
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
