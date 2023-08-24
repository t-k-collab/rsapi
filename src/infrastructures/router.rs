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
    interfaces::controllers::members::create_member::CreateMemberController,
    use_cases::members::create_member::CreateMemberInputData,
};

use chrono::{NaiveDate, NaiveDateTime, Utc};

pub fn init_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        // .route("/", get(health_check))
        .route("/members/:member_id", get(get_member))
        .route("/members", post(create_member))
        .with_state(pool)
}

// type ApiResponse<T> = (StatusCode, Json<T>);
struct ApiResponse<T>(StatusCode, Json<T>);

// impl<T> IntoResponse for ApiResponse<T> {
//     fn into_response(self) -> Response {
//         Response::new(body::boxed(self))
//     }
// }

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

// have to rewrite.
#[derive(FromRow, Debug)]
struct SMemberModel {
    // pub member_id: u32,
    pub first_name: String,
    pub middle_name: String,
    pub family_name: String,
    // pub date_of_birth: NaiveDate,
    pub email: String,
    pub password: String,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

async fn get_member(
    State(pool): State<PgPool>,
    // Path((member_id, first_name)): Path<(u32, String)>,
    Path(first_name): Path<String>,
) -> impl IntoResponse {
    {
        // let row = sqlx::query_as("SELECT * FROM members WHERE member_id = $1")
        //     .bind(member_id)
        //     .fetch_one(&pool)
        //     .await;

        let row =
            sqlx::query_as::<Postgres, SMemberModel>("SELECT * FROM members WHERE first_name = $1")
                .bind(first_name)
                .fetch_one(&pool)
                .await;
        println!("{:#?}", row);
        let member = row.unwrap();

        // let output_data = CreateMemberController::create_member(
        //     payload.family_name,
        //     Some(payload.middle_name),
        //     payload.first_name,
        //     payload.pass_code,
        // )
        // .member;
        let utc = Utc::now();
        let output_data = MemberEntity {
            member_id: 111,
            first_name: member.first_name,
            middle_name: member.middle_name,
            family_name: member.family_name,
            // date_of_birth: utc.date_naive(),
            // email: member.email,
            // password: member.password,
            // created_at: utc.naive_utc(),
            // updated_at: utc.naive_utc(),
        };

        (StatusCode::OK, Json(output_data))
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

    (StatusCode::CREATED, Json(output_data))
    // ApiResponse(StatusCode::CREATED, Json(output_data)).into_response()
    // Response::new((StatusCode::CREATED, Json(output_data)))
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
