mod config;
mod error;
mod handler;
mod models;

pub use crate::config::AppConfig;
use crate::handler::{
    create_chat_handler, delete_chat_handler, index_handler, list_chat_handler,
    list_meaasge_handler, send_meaasge_handler, signin_handler, signup_handler,
    update_chat_handler,
};
use axum::{
    routing::{get, patch, post},
    Router,
};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: AppStateInner,
}
impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: AppStateInner { config },
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct AppStateInner {
    config: AppConfig,
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_meaasge_handler),
        )
        .route("/chat:id/message", get(list_meaasge_handler));

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}
