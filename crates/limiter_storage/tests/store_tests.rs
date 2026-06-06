use limiter_storage::memory::memory_store::{MemoryStore, StoredValue};
use limiter_storage::store::{QueryDatabase, timestamp};
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_inserts_new_value_when_key_missing() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        QueryDatabase::create(&store, "k1".to_string(), 10, None);

        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, Some(10));

        // Also verify internal representation
        let map = store.map.read().unwrap();
        let stored = map.get("k1").unwrap();
        assert_eq!(stored.value, 10);
        assert_eq!(stored.expires_at, None);
    }

    #[test]
    fn create_does_not_overwrite_existing_key() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        QueryDatabase::create(&store, "k1".to_string(), 10, None);
        QueryDatabase::create(&store, "k1".to_string(), 99, Some(123));

        // Since create uses entry().or_insert_with(...), the first value should remain
        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, Some(10));

        let map = store.map.read().unwrap();
        let stored = map.get("k1").unwrap();
        assert_eq!(stored.value, 10);
        assert_eq!(stored.expires_at, None);
    }

    #[test]
    fn find_returns_none_for_missing_key() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        let got = QueryDatabase::find(&store, "missing".to_string());
        assert_eq!(got, None);
    }

    #[test]
    fn find_returns_value_when_not_expired() {
        let store: MemoryStore<String, String> = MemoryStore::new();

        let now = timestamp();
        QueryDatabase::create(
            &store,
            "k1".to_string(),
            "value".to_string(),
            Some(now + 60),
        );

        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, Some("value".to_string()));
    }

    // #[test]
    // fn find_returns_none_and_deletes_when_expired_in_past() {
    //     let store: MemoryStore<String, u64> = MemoryStore::new();

    //     let now = timestamp();
    //     QueryDatabase::create(&store, "k1".to_string(), 10, Some(now.saturating_sub(1)));

    //     let got = QueryDatabase::find(&store, "k1".to_string());
    //     assert_eq!(got, None);

    //     // Side effect: it should delete the expired key
    //     let map = store.map.read().unwrap();
    //     assert!(!map.contains_key("k1"));
    // }

    // #[test]
    // fn find_treats_expires_at_equal_now_as_expired_and_deletes() {
    //     let store: MemoryStore<String, u64> = MemoryStore::new();

    //     let now = timestamp();
    //     QueryDatabase::create(&store, "k1".to_string(), 10, Some(now));

    //     let got = QueryDatabase::find(&store, "k1".to_string());
    //     assert_eq!(got, None);

    //     let map = store.map.read().unwrap();
    //     assert!(!map.contains_key("k1"));
    // }

    #[test]
    fn delete_bucket_removes_key_and_is_idempotent() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        QueryDatabase::create(&store, "k1".to_string(), 10, None);
        QueryDatabase::delete_bucket(&store, "k1".to_string());

        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, None);

        // idempotent: deleting again should not panic
        QueryDatabase::delete_bucket(&store, "k1".to_string());
    }

    #[test]
    fn update_bucket_status_overwrites_existing_value_and_ttl() {
        let store: MemoryStore<String, u64> = MemoryStore::new();
        let now = timestamp();

        QueryDatabase::create(&store, "k1".to_string(), 10, Some(now + 100));

        QueryDatabase::update_bucket_status(&store, "k1".to_string(), 77, Some(now + 200));

        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, Some(77));

        let map = store.map.read().unwrap();
        let stored = map.get("k1").unwrap();
        assert_eq!(stored.value, 77);
        assert_eq!(stored.expires_at, Some(now + 200));
    }

    #[test]
    fn update_bucket_status_inserts_when_key_missing() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        QueryDatabase::update_bucket_status(&store, "k1".to_string(), 55, None);

        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, Some(55));
    }

    #[test]
    fn find_does_not_delete_when_no_expiration_set() {
        let store: MemoryStore<String, u64> = MemoryStore::new();

        QueryDatabase::create(&store, "k1".to_string(), 10, None);

        let got = QueryDatabase::find(&store, "k1".to_string());
        assert_eq!(got, Some(10));

        let map = store.map.read().unwrap();
        assert!(map.contains_key("k1"));
    }

    #[test]
    fn concurrent_creates_only_one_wins_due_to_or_insert_with() {
        let store: Arc<MemoryStore<String, u64>> = Arc::new(MemoryStore::new());

        let mut handles = vec![];
        for i in 0..10u64 {
            let s = Arc::clone(&store);
            handles.push(thread::spawn(move || {
                QueryDatabase::create(&*s, "same".to_string(), i, None);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }

        // Value should be one of [0..9], but must exist and key count must be 1.
        let map = store.map.read().unwrap();
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("same"));
    }

    #[test]
    fn concurrent_update_and_find_basic_sanity() {
        let store: Arc<MemoryStore<String, u64>> = Arc::new(MemoryStore::new());

        QueryDatabase::create(&*store, "k1".to_string(), 1, None);

        let updater = {
            let s = Arc::clone(&store);
            thread::spawn(move || {
                for v in 2..200u64 {
                    QueryDatabase::update_bucket_status(&*s, "k1".to_string(), v, None);
                }
            })
        };

        let reader = {
            let s = Arc::clone(&store);
            thread::spawn(move || {
                // Just ensure no panic / deadlock and returned value is always Some (no TTL).
                for _ in 0..200 {
                    let got = QueryDatabase::find(&*s, "k1".to_string());
                    assert!(got.is_some());
                }
            })
        };

        updater.join().unwrap();
        reader.join().unwrap();
    }
}
