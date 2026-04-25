mod gen_log_id;

pub use gen_log_id::gen_log_id;

use crate::context::{LogId, task_local::LOG_ID};

pub fn get_or_default_log_id() -> LogId {
    get_log_id().unwrap_or(LogId::from("-"))
}

pub fn get_log_id() -> Option<LogId> {
    LOG_ID
        .try_with(|v| {
            let b = v.borrow();

            b.clone()
        })
        .unwrap_or(None)
}
