use std::fmt::{Display, Formatter};

use axum::{
    body::{Body, Bytes},
    extract::Request,
    middleware::Next,
    response::Response,
};
use http_body_util::BodyExt as _;
use jiff::{SignedDuration, Timestamp};

use crate::error::{AppErrorBuilt, AppResult};

const MAX_LOG_BYTES: usize = 8 * 1024;

pub async fn request_middleware(req: Request, next: Next) -> AppResult<Response> {
    let start_time = Timestamp::now();

    let (parts, body) = req.into_parts();

    let method = parts.method.to_string();

    let uri = parts.uri.to_string();

    let headers = parts.headers.clone();

    log::info!("request {} {} \n headers: {:?}", method, uri, headers);

    let bytes = buffer_and_print("request", body, None).await?;

    let request = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(request).await;

    let (parts, body) = res.into_parts();

    let duration = Timestamp::now().duration_since(start_time);

    let bytes = buffer_and_print("response", body, Some(duration)).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(
    direction: &str,
    body: B,
    duration: Option<SignedDuration>,
) -> AppResult<Bytes>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err(AppErrorBuilt::invalid_param(format!(
                "failed to read {direction} body: {err}"
            )));
        }
    };

    let slice = if bytes.len() > MAX_LOG_BYTES {
        &bytes[..MAX_LOG_BYTES]
    } else {
        &bytes
    };

    let mut body_str = String::from_utf8_lossy(slice).to_string();

    if bytes.len() > MAX_LOG_BYTES {
        body_str.push_str(" ...[truncated]");
    }

    let mut info = format!("{direction} body = {body_str:?}");

    if let Some(dd) = duration {
        info.push_str(format!(" cost: {}", Latency(dd)).as_str())
    }

    log::info!("{}", info);

    Ok(bytes)
}

struct Latency(SignedDuration);

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ms = self.0.as_millis();

        if ms > 0 {
            write!(f, "{} ms", ms)
        } else {
            write!(f, "{} us", self.0.as_micros())
        }
    }
}
