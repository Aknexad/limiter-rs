use limiter_storage::memory::memory_store::MemoryStore;
use limiter_storage::memory::memory_store::StoredValue;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_store() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        let map = store.map.read().unwrap();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn store_allows_manual_insert_and_read() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        {
            let mut map = store.map.write().unwrap();
            map.insert(
                "user1".to_string(),
                StoredValue {
                    value: 42,
                    expires_at: None,
                },
            );
        }

        let map = store.map.read().unwrap();
        let stored = map.get("user1").expect("value should exist");

        assert_eq!(stored.value, 42);
        assert_eq!(stored.expires_at, None);
    }

    #[test]
    fn store_supports_value_with_expiration() {
        let store: MemoryStore<String, String> = MemoryStore::new();

        {
            let mut map = store.map.write().unwrap();
            map.insert(
                "session1".to_string(),
                StoredValue {
                    value: "active".to_string(),
                    expires_at: Some(1_700_000_000),
                },
            );
        }

        let map = store.map.read().unwrap();
        let stored = map.get("session1").expect("value should exist");

        assert_eq!(stored.value, "active");
        assert_eq!(stored.expires_at, Some(1_700_000_000));
    }

    #[test]
    fn store_can_hold_multiple_keys() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        {
            let mut map = store.map.write().unwrap();
            map.insert(
                "a".to_string(),
                StoredValue {
                    value: 1,
                    expires_at: None,
                },
            );
            map.insert(
                "b".to_string(),
                StoredValue {
                    value: 2,
                    expires_at: Some(99),
                },
            );
            map.insert(
                "c".to_string(),
                StoredValue {
                    value: 3,
                    expires_at: None,
                },
            );
        }

        let map = store.map.read().unwrap();
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a").unwrap().value, 1);
        assert_eq!(map.get("b").unwrap().value, 2);
        assert_eq!(map.get("b").unwrap().expires_at, Some(99));
        assert_eq!(map.get("c").unwrap().value, 3);
    }

    #[test]
    fn inserting_same_key_overwrites_previous_value() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        {
            let mut map = store.map.write().unwrap();
            map.insert(
                "dup".to_string(),
                StoredValue {
                    value: 10,
                    expires_at: None,
                },
            );
            map.insert(
                "dup".to_string(),
                StoredValue {
                    value: 20,
                    expires_at: Some(500),
                },
            );
        }

        let map = store.map.read().unwrap();
        assert_eq!(map.len(), 1);

        let stored = map.get("dup").unwrap();
        assert_eq!(stored.value, 20);
        assert_eq!(stored.expires_at, Some(500));
    }

    #[test]
    fn read_lock_can_be_acquired_multiple_times() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        let read1 = store.map.read().unwrap();
        let read2 = store.map.read().unwrap();

        assert!(read1.is_empty());
        assert!(read2.is_empty());
    }

    #[test]
    fn write_lock_allows_mutation() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        {
            let mut map = store.map.write().unwrap();
            map.insert(
                "key".to_string(),
                StoredValue {
                    value: 123,
                    expires_at: None,
                },
            );
        }

        let map = store.map.read().unwrap();
        assert_eq!(map.get("key").unwrap().value, 123);
    }
}
