use std::{sync::Arc, time::Duration};

use base::error::{AppErrorBuilt, AppResult};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use moka::future::Cache;

use crate::cache::{
    CacheManagerTrait, CacheTrait,
    operation::{CacheGenericOperation, CacheManager},
};

async fn new_redis_client_pool(redis_url: &str) -> AppResult<Pool<RedisConnectionManager>> {
    let manager = RedisConnectionManager::new(redis_url).unwrap();

    let pool = Pool::builder()
        .max_size(2000) // 最大连接数
        .min_idle(Some(100)) // 最小空闲连接数（保持2个空闲连接，减少创建连接开销）
        .connection_timeout(Duration::from_secs(3)) // 获取连接的超时时间：3秒内获取不到则报错
        .build(manager)
        .await
        .map_err(|e| AppErrorBuilt::db_common(format!("redis pool create failed: {:?}", e)))?;

    Ok(pool)
}

async fn new_local_client() -> Cache<String, String> {
    let local_client: Cache<String, String> = Cache::builder().max_capacity(10000).build();

    local_client
}

pub async fn new_cache_manager(redis_addr: &str) -> Arc<dyn CacheManagerTrait> {
    let redis_client = new_redis_client_pool(redis_addr).await.unwrap();

    let local_client = new_local_client().await;

    CacheManager::new(Arc::new(redis_client), Arc::new(local_client))
}

pub fn new_generic_operation<T>(cache_manager: Arc<dyn CacheManagerTrait>) -> Arc<dyn CacheTrait<T>>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
{
    CacheGenericOperation::<T>::new(cache_manager)
}

#[cfg(test)]

mod tests {

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]

    struct User {
        id: u64,
        name: String,
    }

    #[tokio::test]

    async fn test_new_generic_operation() {
        let cache_manager = new_cache_manager("redis://127.0.0.1:6379").await;

        let user_op = new_generic_operation::<User>(cache_manager);

        let user = User {
            id: 1,
            name: "test".to_string(),
        };

        let key = "user1";

        user_op
            .set(key, user.clone(), Duration::from_secs(10), None)
            .await
            .unwrap();

        let user_from_cache = user_op.get(key, None).await.unwrap();

        println!("user: {:?}", user_from_cache);

        tokio::time::sleep(Duration::from_secs(11)).await;

        let user_from_cache = user_op.get(key, None).await.unwrap();

        println!("expired {:?}", user_from_cache);
    }
}
