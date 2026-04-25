use axum::extract::{FromRequest, Request, rejection::JsonRejection};

use crate::error::AppErrorBuilt;

pub struct Bind<T>(pub T);

impl<S, T> FromRequest<S> for Bind<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppErrorBuilt;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => Err(AppErrorBuilt::param_bind(format!(
                "bind param failed: {}",
                rejection.body_text()
            ))),
        }
    }
}
