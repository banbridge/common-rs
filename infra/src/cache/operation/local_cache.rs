use std::{sync::Arc, time::Duration};

use base::error::AppResult;
use moka::future::Cache;

use crate::cache::BaseCacheTrait;

pub struct LocalCacheClient {
    client: Arc<Cache<String, String>>,
}

impl LocalCacheClient {
    pub fn new(cli: Arc<Cache<String, String>>) -> Arc<Self> {
        Arc::new(LocalCacheClient { client: cli })
    }
}

#[async_trait::async_trait]

impl BaseCacheTrait for LocalCacheClient {
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let res = self.client.get(key).await;

        Ok(res)
    }

    async fn set(&self, key: &str, value: String, _ttl: Duration) -> AppResult<()> {
        self.client.insert(key.to_string(), value).await;

        Ok(())
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        self.client.invalidate(key).await;

        Ok(())
    }
}
