use axum::http::Error;
use serde::{Deserialize, Serialize};
use sqlx::Error as sqlxErr;

// #[derive(Debug)]
// pub enum SqlxError {
//     DataBaseError(sqlxErr),
// }

// new
// pub trait ResponseJson<T> {
//     fn res(code: StatusCode, body: T) -> (StatusCode, Json<T>);
// }

// impl<T, U> ResponseJson<T> for U
// where
//     T: std::fmt::Debug,
// {
//     fn res(code: StatusCode, body: T) -> (StatusCode, Json<T>) {
//         (code, Json(body))
//     }
// }
// until here new

// #[derive(Debug, Serialize)]
// pub enum JsonResponse<T> {
//     OK(T),
//     CustomError(CustomError),
// }

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CustomError {
    SqlxError { msg: String }, // ?? sqlxErr not used
    NotFoundError { msg: String },
    InternalError { msg: String },
    // NotFoundError(NotFoundError),
    // InternalError(InternalError),
}

#[derive(Debug, Serialize)]
pub struct SqlxError {
    pub msg: String, // ?? sqlxErr not used
}

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct InternalError {
    pub msg: String,
}
