use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sql error :{0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("sql error :{0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("jwt error :{0}")]
    JwtError(#[from] jwt_simple::Error),

    // Current user already exists
    #[error("current email already exists:{0}")]
    EmailAlreadyExists(String),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),

    #[error("Invalid email or  password")]
    SiginError(String),
}

impl ErrorOutput {
    pub fn new(err: impl Into<String>) -> Self {
        Self { error: err.into() }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::JwtError(_) => StatusCode::FORBIDDEN,
            Self::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            Self::SiginError(_) => StatusCode::FORBIDDEN,
        };

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
