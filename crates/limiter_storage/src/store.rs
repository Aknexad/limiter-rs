use crate::memory::memory_store::{MemoryStore, StoredValue};
use crate::redis::redis_storage::RedisStorage;
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

// in memory
pub trait SyncMemoryQueryDatabase<K, V> {
    fn create(&self, id: K, data: V, expires_at: Option<u64>);
    fn find(&self, id: K) -> Option<V>
    where
        V: Clone;

    fn update_bucket_status(&self, id: K, data: V, expires_at: Option<u64>);
    fn delete_bucket(&self, id: K);
}

impl<K, V> SyncMemoryQueryDatabase<K, V> for MemoryStore<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn create(&self, id: K, data: V, expires_at: Option<u64>) {
        let mut write_db = self.map.write().unwrap();

        let storage_value = StoredValue::<V> {
            value: data,
            expires_at,
        };

        write_db.entry(id).or_insert_with(|| storage_value);
    }

    fn find(&self, id: K) -> Option<V>
    where
        V: Clone,
    {
        let store = self.map.read().unwrap();

        let bucket_data = store.get(&id)?;

        if let Some(exp) = bucket_data.expires_at {
            let current_time = timestamp();

            if current_time >= exp {
                self.delete_bucket(id);
                return None;
            }
        }

        Some(bucket_data.value.clone())
    }

    fn update_bucket_status(&self, id: K, data: V, expires_at: Option<u64>) {
        let mut bucket = self.map.write().unwrap();

        bucket.remove(&id);

        let storage_value = StoredValue::<V> {
            value: data,
            expires_at,
        };

        bucket.insert(id, storage_value);
    }

    fn delete_bucket(&self, id: K) {
        let mut bucket = self.map.write().unwrap();

        bucket.remove(&id);
    }
}

// REDIS

pub trait AsyncRedisQueryDatabase<V>
where
    V: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    fn create(
        &self,
        id: String,
        data: V,
        expires_at: Option<u64>,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send;
    fn find(&self, id: String) -> impl Future<Output = Option<V>> + Send
    where
        V: Clone + Send;

    fn update_bucket_status(
        &self,
        id: String,
        data: V,
        expires_at: Option<u64>,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send;
    fn delete_bucket(
        &self,
        id: String,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send;
}
impl<V> AsyncRedisQueryDatabase<V> for RedisStorage
where
    V: Serialize + DeserializeOwned + Clone + Send + Sync + 'static + redis::ToSingleRedisArg,
{
    fn create(
        &self,
        id: String,
        data: V,
        expires_at: Option<u64>,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send {
        async move {
            let value = serde_json::to_string(&data)?;
            let ttl = expires_at.unwrap_or(3600);

            let _: () =
                AsyncCommands::set_ex(&mut self.connections_manager.clone(), id, value, ttl)
                    .await?;

            Ok(())
        }
    }

    fn find(&self, id: String) -> impl Future<Output = Option<V>> + Send
    where
        V: Clone + Send,
    {
        async move {
            let mut connection = self.connections_manager.clone();

            let data = AsyncCommands::get::<_, Option<String>>(&mut connection, &id).await;

            match data {
                Ok(Some(raw)) => serde_json::from_str(&raw).ok(),
                Ok(None) => None,
                Err(err) => {
                    println!("error: {}", err);
                    None
                }
            }
        }
    }

    fn update_bucket_status(
        &self,
        id: String,
        data: V,
        expires_at: Option<u64>,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send {
        let mut con = self.connections_manager.clone();
        async move {
            let value: String = serde_json::to_string(&data)?;
            let ttl = expires_at.unwrap_or(3600);

            let _: () = AsyncCommands::set_ex(&mut con, id, value, ttl).await?;

            Ok(())
        }
    }

    fn delete_bucket(
        &self,
        id: String,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send {
        let mut con = self.connections_manager.clone();
        async move {
            let _: () = AsyncCommands::del(&mut con, id).await?;

            Ok(())
        }
    }
}

pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
