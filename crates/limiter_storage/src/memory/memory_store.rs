use std::collections::HashMap;
use std::sync::RwLock;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoryStore<K, V> {
    pub map: RwLock<HashMap<K, V>>,
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

    // pub fn  insert(&self, key: K, value: V) {
    //     let mut w = self.map.write().unwrap();
    //     w.insert(key, value);
    // }

    // pub fn get(&self, key: &K) -> Option<V>
    // where V: Clone
    // {
    //     let r = self.map.read().unwrap();
    //     r.get(key).cloned()
    // }
}
