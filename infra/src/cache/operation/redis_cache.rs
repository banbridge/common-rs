use std::{sync::Arc, time::Duration};

use base::error::{AppErrorBuilt, AppResult};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncTypedCommands;

use crate::cache::BaseCacheTrait;

pub struct RedisCacheClient {
    client: Arc<Pool<RedisConnectionManager>>,
}

impl RedisCacheClient {
    pub fn new(client: Arc<Pool<RedisConnectionManager>>) -> Arc<RedisCacheClient> {
        Arc::new(RedisCacheClient { client })
    }
}

#[async_trait::async_trait]

impl BaseCacheTrait for RedisCacheClient {
    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let mut conn = self.client.get().await.map_err(|e| {
            AppErrorBuilt::cache_connection_failed(format!("redis get conn failed: {:?}", e))
        })?;

        let value: Option<String> = conn.get(key).await.map_err(|e| {
            AppErrorBuilt::cache_query_failed(format!("redis get key failed: {:?}", e))
        })?;

        Ok(value)
    }

    async fn set(&self, key: &str, value: String, expire: Duration) -> AppResult<()> {
        let mut conn = self.client.get().await.map_err(|e| {
            AppErrorBuilt::cache_connection_failed(format!("redis get conn failed: {:?}", e))
        })?;

        conn.set_ex(key, value, expire.as_secs())
            .await
            .map_err(|e| {
                AppErrorBuilt::cache_set_failed(format!("redis set key failed: {:?}", e))
            })?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        let mut conn = self.client.get().await.map_err(|e| {
            AppErrorBuilt::cache_connection_failed(format!("redis get conn failed: {:?}", e))
        })?;

        let _: usize = conn.del(key).await.map_err(|e| {
            AppErrorBuilt::cache_delete_failed(format!("redis del key failed: {:?}", e))
        })?;

        Ok(())
    }
}
