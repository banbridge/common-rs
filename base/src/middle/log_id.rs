use std::cell::RefCell;

use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    context::{LogId, REQUEST_ID_HEADER, task_local::LOG_ID},
    log_id,
};

pub async fn set_log_id_middleware(mut req: Request, next: Next) -> Response {
    // Log the request id as generated.
    let request_id_header = req.headers().get(REQUEST_ID_HEADER).cloned();

    let mut log_id_scope: Option<LogId> = None;

    if request_id_header.is_none() {
        let log_id_gen = log_id::gen_log_id();

        let log_id_str = HeaderValue::from_str(&log_id_gen.as_str());

        log_id_scope = Some(log_id_gen);

        if let Ok(log_id_header) = log_id_str {
            req.headers_mut().insert(REQUEST_ID_HEADER, log_id_header);
        }
    } else if let Some(v) = request_id_header {
        log_id_scope = Some(LogId::from(v.to_str().unwrap_or("").to_string()));
    }

    LOG_ID
        .scope(RefCell::new(log_id_scope), next.run(req))
        .await
        .into_response()
}
