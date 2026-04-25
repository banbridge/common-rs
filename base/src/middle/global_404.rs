use crate::error::{AppErrorBuilt, AppResult};

pub async fn handler_404() -> AppResult<()> {
    Err(AppErrorBuilt::invalid_param("page not found".to_string()))
}
