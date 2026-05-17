mod algorithms;

pub trait RateLimiter {
    fn allow(&self, key: &str) -> bool;
}

pub fn debug() {
    let user_bucket = algorithms::token_bucket::TokenBucket::new();

    println!("user bucket data => {:?}", user_bucket);

    println!("user bucket capasity ==> {}", user_bucket.capacity);
}

// cargo run --example demo -p limiter-core
