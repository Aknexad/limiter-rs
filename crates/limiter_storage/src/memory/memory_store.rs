use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct StoredValue<V> {
    pub value: V,
    pub expires_at: Option<u64>,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoryStore<K, V> {
    pub map: RwLock<HashMap<K, StoredValue<V>>>,
}

impl<K, V> MemoryStore<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}
