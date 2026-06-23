use crate::{
    error::AppError,
    models::{CreateUser, SignUser, User},
    AppState,
};
use anyhow::Result;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AuthOutput {
    token: String,
}
#[axum::debug_handler]
pub(crate) async fn signin_handler(
    State(stare): State<AppState>,
    Json(input): Json<SignUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::verify(input, &stare.pool).await?;
    match user {
        Some(user) => {
            let token = stare.ek.sign(user)?;
            let body = Json(AuthOutput { token });
            Ok(body)
        }

        None => Err(AppError::SiginError(
            "Invalid email or password".to_string(),
        )),
    }
}

pub(crate) async fn signup_handler(
    State(stare): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(input, &stare.pool).await?;
    let token = stare.ek.sign(user)?;
    let body = Json(AuthOutput { token });
    Ok(body)
}
