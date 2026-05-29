use limiter_core::{algorithms, limiter::RateLimiter, policy};
use limiter_storage::{memory::memory_store::MemoryStore, store::QueryDatabase};

fn main() {
    let config = policy::RatelimiterConfig {
        token_bucket_max_capacity: 100,
        token_bucket_refill_rate: 1,
        bucket_ttl_seconds: 300,
    };

    let user_bucket = algorithms::token_bucket::TokenBucket::new(
        config.token_bucket_max_capacity,
        config.token_bucket_refill_rate,
    );

    let bucket_id: String = "1".to_string();
    let db = MemoryStore::new();

    let result = user_bucket.check_rate(bucket_id.clone(), 5, &db);

    println!("result of request is {}", result);
    let bs = db.find(bucket_id).unwrap();
    println!("bucket after update => {:?}", bs);
}

// cargo run --example demo -p limiter_core
