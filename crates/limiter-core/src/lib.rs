pub trait RateLimiter {
    fn allow(&self, key: &str) -> bool;
}
