use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

use crate::error::{AppErrorBuilt, AppResult};

pub fn load_config<'a, T>(file_name: &str, format: FileFormat) -> AppResult<T>
where
    T: Deserialize<'a>,
{
    let settings = Config::builder()
        .add_source(File::with_name(file_name).format(format))
        .add_source(
            Environment::with_prefix("APP")
                .separator("__")
                .try_parsing(true),
        )
        .build()
        .map_err(|e| {
            AppErrorBuilt::invalid_param("load config failed".to_string()).with_base(e.into())
        })?
        .try_deserialize::<T>()
        .map_err(|e| {
            AppErrorBuilt::invalid_param("failed to deserialize app conf".to_string())
                .with_base(e.into())
        })?;

    Ok(settings)
}
