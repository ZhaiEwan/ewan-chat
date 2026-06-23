mod config;
mod error;
mod handler;
mod middlewares;
mod models;
mod utils;

use crate::{
    error::AppError,
    handler::{
        create_chat_handler, delete_chat_handler, index_handler, list_chat_handler,
        list_meaasge_handler, send_meaasge_handler, signin_handler, signup_handler,
        update_chat_handler,
    },
    middlewares::{set_layer, verify_token},
    utils::{DecodingKey, EncodingKey},
};
use anyhow::Result;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}
impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AppState {
    pub fn try_new(config: AppConfig) -> Self {
        let ek = EncodingKey::load(&config.auth.ek).expect("load ek error");
        let dk = DecodingKey::load(&config.auth.dk).expect("load dk error");

        let pool = PgPool::connect_lazy(&config.service.db_url).expect("connect to db failed");
        Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct AppStateInner {
    config: AppConfig,
    ek: EncodingKey,
    dk: DecodingKey,
    pool: PgPool,
}

pub fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config);
    let api = Router::new()
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_meaasge_handler),
        )
        .route("/chat:id/message", get(list_meaasge_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);
    Ok(set_layer(app))
}
