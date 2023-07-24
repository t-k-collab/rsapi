use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{
    entities::member::MemberEntity,
    interfaces::controllers::members::create_member::CreateMemberController,
    use_cases::members::create_member::CreateMemberInputData,
};

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/members", post(create_member))
}

async fn health_check() {
    {
        let _ = Json(json!({ "healthCheck": "ok" }));
    }
}

async fn create_member(
    Json(payload): Json<CreateMemberInputData>,
) -> (StatusCode, Json<MemberEntity>) {
    println!(
        "{}, {:?}, {}, {}",
        payload.family_name,
        Some(&payload.middle_name),
        payload.last_name,
        payload.pass_code
    );

    let output_data = CreateMemberController::create_member(
        payload.family_name,
        payload.middle_name,
        payload.last_name,
        payload.pass_code,
    );

    (StatusCode::CREATED, Json(output_data.member))
}

// remove here.
#[cfg(test)]
mod tests {
    use super::{CreateMemberInputData, MemberEntity};

    #[test]
    fn test_sample() {
        let req_body = CreateMemberInputData {
            family_name: "Api".to_string(),
            middle_name: Some("mid".to_string()),
            last_name: "Rust".to_string(),
            pass_code: "passcode".to_string(),
        };
        let expected = MemberEntity::new(
            "Api".to_string(),
            Some("mid".to_string()),
            "Rust".to_string(),
            "passcode".to_string(),
        );
        assert_eq!(req_body.family_name, expected.family_name);
    }
}
