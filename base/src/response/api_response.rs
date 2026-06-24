use axum::{Json, http::StatusCode, response::IntoResponse};
use faststr::FastStr;
use serde::Serialize;

use crate::{
    error::{AppErrorBuilt, AppResult},
    log_id,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseMetadata {
    request_id: FastStr,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<AppErrorBuilt>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    response_metadata: ResponseMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        (status_code, Json(self)).into_response()
    }
}

impl IntoResponse for AppErrorBuilt {
    fn into_response(self) -> axum::response::Response {
        ApiResponse::<()>::err(self).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok() -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(None),
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(None),
            data: Some(data),
        }
    }

    pub fn err(err: AppErrorBuilt) -> Self {
        ApiResponse {
            response_metadata: Self::build_metadata(Some(err)),
            data: None,
        }
    }

    fn status_code(&self) -> StatusCode {
        if let Some(err) = &self.response_metadata.error {
            StatusCode::from_u16(err.get_http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        } else {
            StatusCode::OK
        }
    }

    fn build_metadata(biz_error: Option<AppErrorBuilt>) -> ResponseMetadata {
        let log_id = log_id::get_or_default_log_id();
        ResponseMetadata {
            request_id: log_id,
            error: biz_error,
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

pub type ApiResult<T> = Result<ApiResponse<T>, AppErrorBuilt>;
