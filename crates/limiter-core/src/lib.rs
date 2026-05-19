mod algorithms;

pub trait RateLimiter {
    fn allow(&self, key: &str) -> bool;
}

pub fn debug() {
    let user_bucket = algorithms::token_bucket::TokenBucket::new_defult_value();

    println!("user bucket data => {:?}", user_bucket);

    let current_tokens = algorithms::token_bucket::TokenBucket::refill_logic(
        1779194046,
        user_bucket.refill_rate,
        user_bucket.capacity,
    );

    println!("total left for this requst {}", current_tokens);

    let requst_status =
        algorithms::token_bucket::TokenBucket::allow_deny_request(current_tokens, 40);

    println!("resutl of requst {}", requst_status);
}

// cargo run --example demo -p limiter-core
