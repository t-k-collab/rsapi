use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use axum::handler::Handler;

pub async fn health_check() {
    // || async "Hello, World!"
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

pub async fn create_member(Json(payload): Json<CreateMember>) -> (StatusCode, Json<Member>) {
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
