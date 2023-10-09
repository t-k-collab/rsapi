use serde::Serialize;

// REFACTOR better coding
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CustomError {
    SqlxError { msg: String }, // ?? sqlxErr not used
    NotFoundError { msg: String },
    InternalError { msg: String },
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
