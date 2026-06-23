use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;

use crate::middlewares::REQUEST_ID_HEADER;

// const SERVER_TIME_HEADER: &str = "x-server-time";
pub(crate) async fn set_request_id(mut req: Request, next: Next) -> Response {
    let request_id = match req.headers().get(REQUEST_ID_HEADER) {
        Some(id) => id.clone(),
        None => {
            let uuid = uuid::Uuid::now_v7().to_string();
            match HeaderValue::from_str(&uuid) {
                Ok(id) => {
                    req.headers_mut().insert(REQUEST_ID_HEADER, id.clone());
                    id
                }
                Err(e) => {
                    warn!("parse generated request id failed: {}", e);
                    return next.run(req).await;
                }
            }
        }
    };

    let mut res = next.run(req).await;
    res.headers_mut().insert(REQUEST_ID_HEADER, request_id);
    res
}
