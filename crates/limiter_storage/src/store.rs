use crate::memory::memory_store::MemoryStore;

pub trait QueryDatabase<K, V> {
    fn create(&self, id: K, data: V);
    fn find(&self, id: &K) -> Option<V>
    where
        V: Clone;
}

impl<K, V> QueryDatabase<K, V> for MemoryStore<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn create(&self, id: K, data: V) {
        let mut write_db = self.map.write().unwrap();
        write_db.insert(id, data);
    }

    fn find(&self, id: &K) -> Option<V>
    where
        V: Clone,
    {
        let bucket_data = self.map.read().unwrap();
        bucket_data.get(id).cloned()
    }
}
