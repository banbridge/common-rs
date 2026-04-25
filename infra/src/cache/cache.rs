use std::{sync::Arc, time::Duration};

use base::error::AppResult;

pub enum CacheType {
    NoCache,
    Redis,
    Local,
    LocalOnRedis,
}

#[async_trait::async_trait]
pub trait BaseCacheTrait: Send + Sync {
    async fn get(&self, key: &str) -> AppResult<Option<String>>;

    async fn set(&self, key: &str, value: String, expire: Duration) -> AppResult<()>;

    async fn delete(&self, key: &str) -> AppResult<()>;
}

#[async_trait::async_trait]

pub trait CacheTrait<T>: Send + Sync
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
{
    async fn get(&self, key: &str, cache_type: Option<CacheType>) -> AppResult<Option<T>>;

    async fn set(
        &self,
        key: &str,
        value: T,
        expire: Duration,
        cache_type: Option<CacheType>,
    ) -> AppResult<()>;

    async fn delete(&self, key: &str, cache_type: Option<CacheType>) -> AppResult<()>;

    fn get_cache_manager(&self) -> &Arc<dyn CacheManagerTrait>;
}

pub trait CacheManagerTrait: Send + Sync {
    fn get_redis_cache(&self) -> Arc<dyn BaseCacheTrait>;

    fn get_local_cache(&self) -> Arc<dyn BaseCacheTrait>;

    fn get_local_on_redis_cache(&self) -> Arc<dyn BaseCacheTrait>;
}
