use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

mod user;

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq)]
pub(crate) struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
