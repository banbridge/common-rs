use std::cmp::max;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DBManager(DatabaseConnection);

pub async fn get_postgresql_db(dsn: &str) -> anyhow::Result<DBManager> {
    let mut options = ConnectOptions::new(dsn);

    let cpu_cores = num_cpus::get() as u32;

    options
        .min_connections(max(10, cpu_cores))
        .max_connections(max(30, cpu_cores * 8))
        .connect_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(300))
        .max_lifetime(std::time::Duration::from_secs(3600 * 24))
        .sqlx_logging(false);

    let db = Database::connect(options).await?;

    db.ping().await?;

    Ok(DBManager::new(db))
}

impl DBManager {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    pub fn get_db(&self) -> &DatabaseConnection {
        &self.0
    }
}
