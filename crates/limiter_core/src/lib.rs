mod algorithms;
pub mod limiter;
mod models;
mod utils;

use limiter_storage::memory::memory_store::MemoryStore;
use limiter_storage::{self, store::QueryDatabase};

use crate::limiter::RateLimiter;

pub fn debug() {
    let user_bucket = algorithms::token_bucket::TokenBucket::new_default_value();

    let bucket_id = "ab-23".to_string();
    let db = MemoryStore::new();

    // let bucket_status = models::BucketState::new(100, 1779535331);

    // println!("bucket status is {:?}",bucket_status);
    // db.create(bucket_id.clone(), bucket_status);

    let result = user_bucket.allow(bucket_id.clone(), &db);

    println!("result of request is {}", result);
    let bs = db.find(&bucket_id).unwrap();
    println!("bucket after update => {:?}",bs);
}

// cargo run --example demo -p limiter-core

// example of call memory in function
// fn process_user_data<S>(store: &S, user_id: i32)
// where
//     S: Store<i32, UserProfile>
// {
//     if let Some(user) = store.get(&user_id) {
//         println!("Processing: {}", user.username);
//     }
// }
