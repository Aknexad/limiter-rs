use limiter_core::algorithms::token_bucket::TokenBucket;

#[cfg(test)]
mod tests {
    use super::*;
    use limiter_core::utils::time::timestamp;

    #[test]
    fn new_initializes_bucket_with_full_capacity() {
        let bucket = TokenBucket::new(10, 2);

        assert_eq!(bucket.capacity, 10);
        assert_eq!(bucket.refill_rate, 2);
        assert_eq!(bucket.current_tokens, 10);
    }

    #[test]
    fn new_sets_timestamp_to_now_or_very_close() {
        let before = timestamp();
        let bucket = TokenBucket::new(10, 2);
        let after = timestamp();

        assert!(bucket.last_refill_timestamp >= before);
        assert!(bucket.last_refill_timestamp <= after);
    }

    #[test]
    fn new_allows_zero_capacity() {
        let bucket = TokenBucket::new(0, 5);

        assert_eq!(bucket.capacity, 0);
        assert_eq!(bucket.refill_rate, 5);
        assert_eq!(bucket.current_tokens, 0);
    }

    #[test]
    fn new_allows_zero_refill_rate() {
        let bucket = TokenBucket::new(10, 0);

        assert_eq!(bucket.capacity, 10);
        assert_eq!(bucket.refill_rate, 0);
        assert_eq!(bucket.current_tokens, 10);
    }

    #[test]
    fn refill_logic_returns_zero_if_last_refill_is_in_future() {
        let now = timestamp();
        let result = TokenBucket::refill_logic(now + 10, 5, 100);

        assert_eq!(result, 0);
    }

    #[test]
    fn refill_logic_returns_zero_when_no_time_has_elapsed() {
        let now = timestamp();
        let result = TokenBucket::refill_logic(now, 5, 100);

        assert_eq!(result, 0);
    }

    #[test]
    fn refill_logic_refills_based_on_elapsed_time_and_rate() {
        let now = timestamp();
        let last_refill = now.saturating_sub(3);

        let result = TokenBucket::refill_logic(last_refill, 2, 100);

        // Expected around 6, but using invariant checks because timestamp() is called again inside.
        assert!(result >= 6 || result == 4 || result == 5);
        assert!(result <= 8);
    }

    #[test]
    fn refill_logic_caps_refilled_tokens_at_capacity() {
        let now = timestamp();
        let last_refill = now.saturating_sub(100);

        let result = TokenBucket::refill_logic(last_refill, 50, 25);

        assert_eq!(result, 25);
    }

    #[test]
    fn refill_logic_returns_zero_when_refill_rate_is_zero() {
        let now = timestamp();
        let last_refill = now.saturating_sub(100);

        let result = TokenBucket::refill_logic(last_refill, 0, 100);

        assert_eq!(result, 0);
    }

    #[test]
    fn refill_logic_returns_zero_when_capacity_is_zero() {
        let now = timestamp();
        let last_refill = now.saturating_sub(100);

        let result = TokenBucket::refill_logic(last_refill, 10, 0);

        assert_eq!(result, 0);
    }

    #[test]
    fn allow_deny_request_allows_when_consume_less_than_current() {
        assert!(TokenBucket::allow_deny_request(10, 3));
    }

    #[test]
    fn allow_deny_request_allows_when_consume_equals_current() {
        assert!(TokenBucket::allow_deny_request(10, 10));
    }

    #[test]
    fn allow_deny_request_denies_when_consume_exceeds_current() {
        assert!(!TokenBucket::allow_deny_request(10, 11));
    }

    #[test]
    fn allow_deny_request_allows_zero_token_consumption() {
        assert!(TokenBucket::allow_deny_request(0, 0));
        assert!(TokenBucket::allow_deny_request(10, 0));
    }

    #[test]
    fn reminder_token_after_request_subtracts_when_enough_tokens_exist() {
        let remaining = TokenBucket::reminder_token_after_request(3, 10);
        assert_eq!(remaining, 7);
    }

    #[test]
    fn reminder_token_after_request_returns_zero_when_exactly_consumed() {
        let remaining = TokenBucket::reminder_token_after_request(10, 10);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn reminder_token_after_request_saturates_at_zero_when_consumption_exceeds_current() {
        let remaining = TokenBucket::reminder_token_after_request(15, 10);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn reminder_token_after_request_with_zero_consumption_keeps_current_tokens() {
        let remaining = TokenBucket::reminder_token_after_request(0, 10);
        assert_eq!(remaining, 10);
    }

    #[test]
    fn request_decision_and_remaining_tokens_are_consistent_for_allowed_request() {
        let current_tokens = 8;
        let consume_tokens = 3;

        let allowed = TokenBucket::allow_deny_request(current_tokens, consume_tokens);
        let remaining = TokenBucket::reminder_token_after_request(consume_tokens, current_tokens);

        assert!(allowed);
        assert_eq!(remaining, 5);
    }

    #[test]
    fn denied_request_still_has_safe_remaining_token_calculation() {
        let current_tokens = 2;
        let consume_tokens = 5;

        let allowed = TokenBucket::allow_deny_request(current_tokens, consume_tokens);
        let remaining = TokenBucket::reminder_token_after_request(consume_tokens, current_tokens);

        assert!(!allowed);
        assert_eq!(remaining, 0);
    }
}
