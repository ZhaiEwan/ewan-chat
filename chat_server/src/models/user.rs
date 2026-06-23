use std::mem;

use crate::error::AppError;
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Pool, Postgres};
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Clone)]
pub struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    #[sqlx(default)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CreateUser {
    pub(crate) fullname: String,
    pub(crate) email: String,
    pub(crate) password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SignUser {
    pub(crate) email: String,
    pub(crate) password: String,
}
impl User {
    #[allow(unused)]
    pub(crate) async fn find_by_email(
        email: &str,
        pool: &Pool<Postgres>,
    ) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as("select id,fullname,email,created_at from users where email= $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub(crate) async fn create(create_user: CreateUser, pool: &PgPool) -> Result<Self, AppError> {
        let password_hash = hash_password(&create_user.password)?;

        let user = Self::find_by_email(&create_user.email, pool).await?;
        if user.is_some() {
            return Err(AppError::EmailAlreadyExists(create_user.email));
        }
        let user = sqlx::query_as(
            r#"
        INSERT INTO users (email,fullname,password_hash)
        VALUES($1,$2,$3)
        RETURNING id ,fullname, email,created_at
        "#,
        )
        .bind(create_user.email)
        .bind(create_user.fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub(crate) async fn verify(
        sign_user: SignUser,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, password_hash,email,fullname,created_at FROM users WHERE email = $1",
        )
        .bind(sign_user.email)
        .fetch_optional(pool)
        .await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash).unwrap_or_default();
                let is_verify = verify_password(&sign_user.password, &password_hash)?;
                if is_verify {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let verify = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(verify)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn hash_password_and_verify() -> Result<()> {
        let password = "zhai42";
        let password_hash = hash_password(password)?;
        assert_eq!(password_hash.len(), 97);
        assert!(verify_password(password, &password_hash)?);
        Ok(())
    }
}
