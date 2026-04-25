use axum::{Json, http::StatusCode, response::IntoResponse};
use faststr::FastStr;
use serde::Serialize;

use crate::{
    error::{AppErrorBuilt, AppResult},
    log_id,
};

// #[allow(dead_code)]
// pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]

pub struct ApiResponse<T>
where
    T: Serialize,
{
    metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]

pub struct Metadata {
    request_id: FastStr,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<AppErrorBuilt>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), Json(&self)).into_response()
    }
}

impl IntoResponse for AppErrorBuilt {
    fn into_response(self) -> axum::response::Response {
        ApiResponse::<()>::err(self).into_response()
    }
}

#[allow(dead_code)]

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok() -> Self {
        Self {
            metadata: Metadata {
                request_id: log_id::get_or_default_log_id(),
                error: None,
            },
            data: None,
        }
    }

    pub fn err(err: AppErrorBuilt) -> Self {
        Self {
            metadata: Self::build_metadata(Some(err)),
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            metadata: Self::build_metadata(None),
            data: Some(data),
        }
    }

    pub fn get_status_code(&self) -> StatusCode {
        if let Some(err) = &self.metadata.error {
            let err_status_code = err.get_http_status();

            StatusCode::from_u16(err_status_code).unwrap_or(StatusCode::BAD_REQUEST)
        } else {
            StatusCode::OK
        }
    }

    fn build_metadata(err: Option<AppErrorBuilt>) -> Metadata {
        Metadata {
            error: err,
            request_id: log_id::get_or_default_log_id(),
        }
    }
}

impl<T> From<AppResult<T>> for ApiResponse<T>
where
    T: Serialize,
{
    fn from(result: AppResult<T>) -> Self {
        match result {
            Ok(data) => ApiResponse::ok_with_data(data),
            Err(err) => ApiResponse::err(err),
        }
    }
}
