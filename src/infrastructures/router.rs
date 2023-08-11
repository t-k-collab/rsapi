use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sqlx::{PgPool, Pool, Postgres};

use crate::{
    entities::member::MemberEntity,
    interfaces::controllers::members::create_member::CreateMemberController,
    use_cases::members::create_member::CreateMemberInputData,
};

pub fn init_router(pool: Pool<Postgres>) -> Router {
    // pub fn init_router() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/members", post(create_member))
        .with_state(pool)
}

type ApiResponse<T> = (StatusCode, Json<T>);

async fn health_check(State(pool): State<PgPool>) {
    {
        let row = sqlx::query!("SELECT * FROM families")
            .fetch_all(&pool)
            .await;

        let result = match row {
            Ok(res) => (StatusCode::OK, Json(res)),
            Err(msg) => panic!("{:?}", msg),
        };
        let a = &result.1;
        println!("{:#?}", a);
    }
}

async fn create_member(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateMemberInputData>,
) -> ApiResponse<MemberEntity> {
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
