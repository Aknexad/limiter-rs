use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenBucket {
    pub capacity: u64,
    pub refill_rate: u64,
    pub current_tokens: u64,
    pub(crate) last_refill_timestamp: u64, // user pub(crate) for reminder of this mothode
}

impl TokenBucket {
    pub fn new_defult_value() -> Self {
        Self {
            capacity: 100,
            refill_rate: 1,
            current_tokens: 10,
            last_refill_timestamp: Self::timestamp(),
        }
    }

    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            refill_rate,
            current_tokens: capacity,
            last_refill_timestamp: Self::timestamp(),
        }
    }

    pub fn refill_logic(last_refill: u64, refill_rate: u64, capacity: u64) -> u64 {
        let current_time = Self::timestamp();

        let elapsed_time = current_time - last_refill;
        let current_tokens = elapsed_time * refill_rate;

        current_tokens.min(capacity)
    }

    pub fn allow_deny_request(current_tokens: u64, consum_tokens: u64) -> bool {
        consum_tokens <= current_tokens
    }

    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
