use crate::memory::memory_store::{MemoryStore, StoredValue};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait QueryDatabase<K, V> {
    fn create(&self, id: K, data: V, expires_at: Option<u64>);
    fn find(&self, id: K) -> Option<V>
    where
        V: Clone;

    fn update_bucket_status(&self, id: K, data: V, expires_at: Option<u64>);
    fn delete_bucket(&self, id: K);
}

impl<K, V> QueryDatabase<K, V> for MemoryStore<K, V>
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

pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
