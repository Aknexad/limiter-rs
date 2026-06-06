use std::ptr::read;

use limiter_core::{
    algorithms,
    limiter::RateLimiter,
    models::{self, key::RateLimiterInputKey::Ip},
    policy,
};
use limiter_storage::redis;
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

    let input_data = models::key::RateLimiterInputData {
        service_name: "auth".to_string(),
        key: Ip("85.23.11.34".to_string()),
    };

    let key = input_data.convert_to_storage_key();
    let db = MemoryStore::new();

    let result = user_bucket.check_rate(key.clone(), 5, &db);

    println!("result of request is {}", result);
    let bs = db.find(key).unwrap();
    println!("bucket after update => {:?}", bs);
}

// cargo run --example demo -p limiter_core
