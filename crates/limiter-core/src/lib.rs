mod algorithms;

pub trait RateLimiter {
    fn allow(&self, key: &str) -> bool;
}

pub fn debug() {
    let user_bucket = algorithms::token_bucket::TokenBucket::new_defult_value();

    println!("user bucket data => {:?}", user_bucket);

    let refill_token = algorithms::token_bucket::TokenBucket::refill_logic(
        1779130636,
        user_bucket.refill_rate,
        user_bucket.capacity,
    );

    println!("total left for this requst {}", refill_token);
}

// cargo run --example demo -p limiter-core
