use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    id: u16,
    familyName: String,
    lastName: String,
    middleName: String,
    passCode: String,
    created: String,
    updated: String,
}

#[derive(Deserialize)]
pub struct CreateMember {
    id: u16,
}

async fn create_member(Json(payload): Json<CreateMember>) -> (StatusCode, Json<Member>) {
    println!("{}", payload.id);
    let member = Member {
        id: payload.id,
        familyName: "Api".to_string(),
        lastName: "Rust".to_string(),
        middleName: "mid".to_string(),
        passCode: "passcode".to_string(),
        created: "20230720".to_string(),
        updated: "20230720".to_string(),
    };

    (StatusCode::CREATED, Json(member))
}

// remove here.
#[cfg(test)]
mod tests {
    use super::{CreateMember, Member};

    #[test]
    fn test_sample() {
        let req_body = CreateMember { id: 2 };
        let expected = Member {
            id: 2,
            familyName: "Api".to_string(),
            lastName: "Rust".to_string(),
            middleName: "mid".to_string(),
            passCode: "passcode".to_string(),
            created: "20230720".to_string(),
            updated: "20230720".to_string(),
        };
        assert_eq!(req_body.id, expected.id);
    }
}
