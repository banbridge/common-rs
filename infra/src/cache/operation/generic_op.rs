use std::{marker::PhantomData, sync::Arc, time::Duration};

use base::error::{AppErrorBuilt, AppResult};
use serde::{Serialize, de::DeserializeOwned};

use crate::cache::{BaseCacheTrait, CacheManagerTrait, CacheTrait, CacheType};

pub struct CacheGenericOperation<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    manager: Arc<dyn CacheManagerTrait>,
    _marker: PhantomData<T>,
}

impl<T> CacheGenericOperation<T>
where
    T: DeserializeOwned + Serialize + Send + Sync + 'static,
{
    pub fn new(manager: Arc<dyn CacheManagerTrait>) -> Arc<dyn CacheTrait<T>> {
        let res = Self {
            manager,
            _marker: PhantomData,
        };

        Arc::new(res)
    }

    fn get_base_cache(&self, cache_type: Option<CacheType>) -> Arc<dyn BaseCacheTrait> {
        if let Some(cache_type) = cache_type {
            return self.get_base_cache_with_type(cache_type).clone();
        }

        self.manager.get_redis_cache().clone()
    }

    fn get_base_cache_with_type(&self, cache_type: CacheType) -> Arc<dyn BaseCacheTrait> {
        match cache_type {
            CacheType::Redis => self.manager.get_redis_cache().clone(),
            _ => self.manager.get_redis_cache().clone(),
        }
    }
}

#[async_trait::async_trait]

impl<T> CacheTrait<T> for CacheGenericOperation<T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    async fn delete(&self, key: &str, cache_type: Option<CacheType>) -> AppResult<()> {
        let base_cache = self.get_base_cache(cache_type);

        base_cache.delete(key).await
    }

    async fn get(&self, key: &str, cache_type: Option<CacheType>) -> AppResult<Option<T>> {
        let base_cache = self.get_base_cache(cache_type);

        let bytes = base_cache.get(key).await?;

        let value = if let Some(bytes) = bytes {
            Some(serde_json::from_str(&bytes).map_err(|e| {
                AppErrorBuilt::db_common(format!("deserialize cache value failed: {:?}", e))
            })?)
        } else {
            None
        };

        Ok(value)
    }

    async fn set(
        &self,
        key: &str,
        value: T,
        expire: Duration,
        cache_type: Option<CacheType>,
    ) -> AppResult<()> {
        let base_cache = self.get_base_cache(cache_type);

        let bytes = serde_json::to_string(&value).map_err(|e| {
            AppErrorBuilt::db_common(format!("serialize cache value failed: {:?}", e))
        })?;

        base_cache.set(key, bytes, expire).await
    }

    fn get_cache_manager(&self) -> &Arc<dyn CacheManagerTrait> {
        &self.manager
    }
}
