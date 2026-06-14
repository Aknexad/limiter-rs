use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use limiter_core::algorithms::token_bucket::TokenBucket;
use limiter_core::models::bucket_state::BucketState;
use limiter_core::utils::time::timestamp;
use limiter_storage::store::SyncMemoryQueryDatabase;

#[cfg(test)]
mod tests {
    use limiter_core::limiter::RateLimiter;

    use super::*;

    /// A minimal in-memory mock for QueryDatabase<String, BucketState>.
    /// It records create/update calls so tests can assert side effects.
    #[derive(Clone, Default)]
    struct MockStore {
        inner: Arc<Mutex<Inner>>,
    }

    #[derive(Default)]
    struct Inner {
        map: HashMap<String, BucketState>,
        creates: Vec<(String, BucketState)>,
        updates: Vec<(String, BucketState)>,
    }

    impl MockStore {
        fn with_bucket(self, key: &str, state: BucketState) -> Self {
            self.inner
                .lock()
                .unwrap()
                .map
                .insert(key.to_string(), state);
            self
        }

        fn get(&self, key: &str) -> Option<BucketState> {
            self.inner.lock().unwrap().map.get(key).cloned()
        }

        fn creates_len(&self) -> usize {
            self.inner.lock().unwrap().creates.len()
        }

        fn updates_len(&self) -> usize {
            self.inner.lock().unwrap().updates.len()
        }

        fn last_update(&self) -> Option<(String, BucketState)> {
            self.inner.lock().unwrap().updates.last().cloned()
        }
    }

    // ---- Implement the external trait for our mock ----
    //
    // NOTE: I don’t know the exact signature of QueryDatabase.
    // The methods below match what your production code calls:
    // - find(key) -> Option<V>
    // - create(key, value, ttl)
    // - update_bucket_status(key, value, ttl)
    //
    // If your trait differs slightly (e.g. &mut self, Result, different ttl type),
    // tell me and I’ll adjust to compile exactly.

    impl SyncMemoryQueryDatabase<String, BucketState> for MockStore {
        fn find(&self, key: String) -> Option<BucketState> {
            self.inner.lock().unwrap().map.get(&key).cloned()
        }

        fn create(&self, key: String, value: BucketState, _ttl: Option<u64>) {
            let mut g = self.inner.lock().unwrap();
            g.map.insert(key.clone(), value.clone());
            g.creates.push((key, value));
        }

        fn update_bucket_status(&self, key: String, value: BucketState, _ttl: Option<u64>) {
            let mut g = self.inner.lock().unwrap();
            g.map.insert(key.clone(), value.clone());
            g.updates.push((key, value));
        }
        #[warn(unused_variables)]
        fn delete_bucket(&self, id: String) {
            println!("{}", id);
        }
    }

    #[test]
    fn check_rate_creates_new_consumer_when_missing_and_allows_request() {
        let store = MockStore::default();
        let bucket = TokenBucket {
            capacity: 10,
            refill_rate: 1,
            current_tokens: 10,
            last_refill_timestamp: timestamp(),
        };

        let allowed = bucket.check_rate("k1".to_string(), 3, &store);

        assert!(allowed);
        assert_eq!(store.creates_len(), 1);
        assert_eq!(store.updates_len(), 0);

        // Verify stored state
        let saved = store.get("k1").expect("bucket should be created");
        assert_eq!(saved.current_tokens, 7);
    }

    #[test]
    fn check_rate_when_existing_bucket_and_deny_does_not_update_storage() {
        let store = MockStore::default().with_bucket(
            "k1",
            BucketState {
                current_tokens: 0,
                // last_refill set to now so refill_logic returns 0
                last_refill_timestamp: timestamp(),
            },
        );

        let bucket = TokenBucket {
            capacity: 10,
            refill_rate: 1,
            current_tokens: 10,
            last_refill_timestamp: timestamp(),
        };

        let allowed = bucket.check_rate("k1".to_string(), 1, &store);

        assert!(!allowed);
        assert_eq!(store.creates_len(), 0);
        assert_eq!(store.updates_len(), 0);

        // state unchanged
        let saved = store.get("k1").unwrap();
        assert_eq!(saved.current_tokens, 0);
    }

    #[test]
    fn check_rate_when_existing_bucket_and_allow_updates_storage_timestamp_and_tokens_invariant() {
        let old_ts = timestamp().saturating_sub(5);
        let store = MockStore::default().with_bucket(
            "k1",
            BucketState {
                current_tokens: 0,
                last_refill_timestamp: old_ts,
            },
        );

        let bucket = TokenBucket {
            capacity: 10,
            refill_rate: 10, // should refill to capacity quickly
            current_tokens: 10,
            last_refill_timestamp: timestamp(),
        };

        let allowed = bucket.check_rate("k1".to_string(), 1, &store);
        assert!(allowed);
        assert_eq!(store.updates_len(), 1);

        let (_, updated) = store.last_update().unwrap();
        // timestamp should be refreshed (>= old)
        assert!(updated.last_refill_timestamp >= old_ts);
        // tokens should never exceed capacity
        assert!(updated.current_tokens <= bucket.capacity);
    }

    #[test]
    fn create_new_consumer_with_consume_equal_capacity_leaves_zero_tokens() {
        let store = MockStore::default();
        let bucket = TokenBucket {
            capacity: 5,
            refill_rate: 1,
            current_tokens: 5,
            last_refill_timestamp: timestamp(),
        };

        let allowed = bucket.check_rate("k1".to_string(), 5, &store);

        assert!(allowed);
        let saved = store.get("k1").unwrap();
        assert_eq!(saved.current_tokens, 0);
    }
}
