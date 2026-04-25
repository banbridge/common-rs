mod global_404;
mod log_id;
mod request_timer;
mod trace;

use std::time::Duration;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderName, StatusCode},
    middleware,
};
use bytesize::ByteSize;
pub use global_404::*;
pub use request_timer::*;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer, normalize_path::NormalizePathLayer, request_id::PropagateRequestIdLayer,
    timeout::TimeoutLayer,
};
pub use trace::*;

use crate::{context::REQUEST_ID_HEADER, middle::log_id::set_log_id_middleware};

pub fn add_middleware_list(router: Router) -> Router {
    let timeout_l =
        TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30));

    let body_limit_l = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);

    let cors_l = CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false)
        .max_age(Duration::from_secs(3600 * 12));

    let normalize_path_l = NormalizePathLayer::trim_trailing_slash();

    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    let layer = ServiceBuilder::new()
        .layer(middleware::from_fn(set_log_id_middleware))
        .layer(get_trace_layer())
        .layer(cors_l)
        .layer(normalize_path_l)
        .layer(middleware::from_fn(request_middleware))
        .layer(body_limit_l)
        .layer(timeout_l)
        // send headers from request to response headers
        .layer(PropagateRequestIdLayer::new(x_request_id));

    router.layer(layer)
}
