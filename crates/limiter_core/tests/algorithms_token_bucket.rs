use limiter_core::algorithms::token_bucket::TokenBucket;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_new_bucket() {
        let bucket = TokenBucket::new(100, 1);

        assert_eq!(bucket.capacity, 100);
        assert_eq!(bucket.refill_rate, 1);
        assert_eq!(bucket.current_tokens, 100);
    }

    #[test]
    fn max_token_form_refill_logic() {
        
        let last_refill:u64= 1770152662;
        let result = TokenBucket::refill_logic(last_refill, 1, 100);

        assert_eq!(result,100);
    }

    #[test]
    fn zero_token_form_refill_logic() {
        
        let last_refill:u64= 1801663462;
        let result = TokenBucket::refill_logic(last_refill, 1, 100);

        assert_eq!(result,0);
    }
    
}


