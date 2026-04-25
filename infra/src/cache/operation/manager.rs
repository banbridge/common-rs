use std::sync::Arc;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use moka::future::Cache;

use crate::cache::{
    BaseCacheTrait, CacheManagerTrait,
    operation::{local_cache, local_on_redis_cache, redis_cache},
};

pub struct CacheManager {
    redis_operation: Arc<dyn BaseCacheTrait>,
    local_operation: Arc<dyn BaseCacheTrait>,
    local_on_redis_operation: Arc<dyn BaseCacheTrait>,

    _local_client: Arc<Cache<String, String>>,
    _redis_client: Arc<Pool<RedisConnectionManager>>,
}

impl CacheManager {
    pub fn new(
        redis_client: Arc<Pool<RedisConnectionManager>>,
        local_client: Arc<Cache<String, String>>,
    ) -> Arc<Self> {
        let redis_operation = redis_cache::RedisCacheClient::new(redis_client.clone());

        let local_operation = local_cache::LocalCacheClient::new(local_client.clone());

        let local_on_redis_operation = local_on_redis_cache::LocalOnRedisCacheClient::new(
            local_operation.clone(),
            redis_operation.clone(),
        );

        Arc::new(Self {
            redis_operation,
            local_operation,
            local_on_redis_operation,
            _redis_client: redis_client,
            _local_client: local_client,
        })
    }
}

impl CacheManagerTrait for CacheManager {
    fn get_redis_cache(&self) -> Arc<dyn BaseCacheTrait> {
        self.redis_operation.clone()
    }

    fn get_local_cache(&self) -> Arc<dyn BaseCacheTrait> {
        self.local_operation.clone()
    }

    fn get_local_on_redis_cache(&self) -> Arc<dyn BaseCacheTrait> {
        self.local_on_redis_operation.clone()
    }
}
