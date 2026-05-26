use limiter_core::{algorithms, limiter::RateLimiter};
use limiter_storage::{memory::memory_store::MemoryStore, store::QueryDatabase};

fn main() {

      let user_bucket = algorithms::token_bucket::TokenBucket::new_default_value();

    let bucket_id: String = "1".to_string();
    let db = MemoryStore::new();

    let result = user_bucket.check_rate(bucket_id.clone(), 5, &db);

    println!("result of request is {}", result);
    let bs = db.find(&bucket_id).unwrap();
    println!("bucket after update => {:?}", bs);
}

// cargo run --example demo -p limiter-core
