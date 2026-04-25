use derive_builder::UninitializedFieldError;
use tokio::time::error::Elapsed;
use validator::ValidationErrors;

use crate::error::AppErrorBuilt;

impl From<UninitializedFieldError> for AppErrorBuilt {
    fn from(value: UninitializedFieldError) -> Self {
        AppErrorBuilt::uninitialized_field_error(format!("some field uninitialized {}", value))
    }
}

impl From<ValidationErrors> for AppErrorBuilt {
    fn from(value: ValidationErrors) -> Self {
        AppErrorBuilt::validate_param_failed(format!("validate err: {:?}", value))
    }
}

impl From<std::io::Error> for AppErrorBuilt {
    fn from(value: std::io::Error) -> Self {
        AppErrorBuilt::command_execute_error(format!("command execute error: {}", value))
    }
}

impl From<Elapsed> for AppErrorBuilt {
    fn from(_value: Elapsed) -> Self {
        AppErrorBuilt::command_execute_timeout("command execute timeout".to_string())
    }
}
