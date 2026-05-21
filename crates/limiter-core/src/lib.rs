mod algorithms;
mod utils;

pub trait RateLimiter {
    fn allow(&self, key: &str) -> bool;

    fn message(&self) {
        println!("a request resice !");
    }
    fn status(&self) -> String;
}

impl RateLimiter for algorithms::token_bucket::TokenBucket {
    fn allow(&self, key: &str) -> bool {
        println!("key  is {}", key);
        let current_tokens = algorithms::token_bucket::TokenBucket::refill_logic(
            1779304074,
            self.refill_rate,
            self.capacity,
        );

        println!("total left for this requst {}", current_tokens);

        algorithms::token_bucket::TokenBucket::allow_deny_request(current_tokens, 39)
    }
    fn status(&self) -> String {
        format!(
            "total capacity {} and last refill was in {}",
            self.capacity, self.last_refill_timestamp
        )
    }
}

pub fn debug() {
    let user_bucket = algorithms::token_bucket::TokenBucket::new_defult_value();

    user_bucket.message();

    println!("user bucket data => {:?}", user_bucket);

    println!("trait call {}", user_bucket.status());

    println!("resutl of requst {}", user_bucket.allow("fj-23"));
}

// cargo run --example demo -p limiter-core
