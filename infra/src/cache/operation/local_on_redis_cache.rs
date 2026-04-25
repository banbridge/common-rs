use std::{sync::Arc, time::Duration};

use base::error::AppResult;

use crate::cache::BaseCacheTrait;

pub struct LocalOnRedisCacheClient {
    local: Arc<dyn BaseCacheTrait>,
    redis: Arc<dyn BaseCacheTrait>,
}

fn wrap_local_key(key: &str) -> String {
    format!("local_on_redis:{}", key)
}

impl LocalOnRedisCacheClient {
    pub fn new(local: Arc<dyn BaseCacheTrait>, redis: Arc<dyn BaseCacheTrait>) -> Arc<Self> {
        Arc::new(Self { local, redis })
    }
}

#[async_trait::async_trait]

impl BaseCacheTrait for LocalOnRedisCacheClient {
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let local_key = wrap_local_key(key);

        let value = self.local.get(local_key.as_str()).await?;

        if value.is_some() {
            return Ok(value);
        }

        let value = self.redis.get(key).await?;

        Ok(value)
    }

    async fn set(&self, key: &str, value: String, expire: Duration) -> AppResult<()> {
        let local_key = wrap_local_key(key);

        self.local
            .set(local_key.as_str(), value.clone(), expire)
            .await?;

        self.redis.set(key, value, expire).await?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        let local_key = wrap_local_key(key);

        self.local.delete(local_key.as_str()).await?;

        self.redis.delete(key).await?;

        Ok(())
    }
}
