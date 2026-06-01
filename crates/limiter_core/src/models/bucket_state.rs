// use std::time;

#[derive(Debug, Clone)]
pub struct BucketState {
    pub current_tokens: u64,
    pub last_refill_timestamp: u64,
}

impl BucketState {
    pub fn new(tokens: u64, timestamp: u64) -> Self {
        Self {
            current_tokens: tokens,
            last_refill_timestamp: timestamp,
        }
    }
}
