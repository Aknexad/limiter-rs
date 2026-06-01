use limiter_core::models::bucket_state::BucketState;
use limiter_core::models::key::{RateLimiterInputData, RateLimiterInputKey};

#[cfg(test)]

mod bucket_state_tests {
    use super::*;

    #[test]
    fn new_sets_fields_correctly_for_typical_values() {
        let state = BucketState::new(10, 1_700_000_000);

        assert_eq!(state.current_tokens, 10);
        assert_eq!(state.last_refill_timestamp, 1_700_000_000);
    }

    #[test]
    fn new_allows_zero_tokens_and_zero_timestamp() {
        let state = BucketState::new(0, 0);

        assert_eq!(state.current_tokens, 0);
        assert_eq!(state.last_refill_timestamp, 0);
    }

    #[test]
    fn new_handles_large_values() {
        let state = BucketState::new(u64::MAX, u64::MAX);

        assert_eq!(state.current_tokens, u64::MAX);
        assert_eq!(state.last_refill_timestamp, u64::MAX);
    }

    #[test]
    fn clone_preserves_all_field_values() {
        let original = BucketState::new(42, 123456789);
        let cloned = original.clone();

        assert_eq!(cloned.current_tokens, original.current_tokens);
        assert_eq!(cloned.last_refill_timestamp, original.last_refill_timestamp);
    }

    #[test]
    fn debug_output_contains_field_names_and_values() {
        let state = BucketState::new(7, 999);
        let debug_str = format!("{:?}", state);

        assert!(debug_str.contains("BucketState"));
        assert!(debug_str.contains("current_tokens"));
        assert!(debug_str.contains("7"));
        assert!(debug_str.contains("last_refill_timestamp"));
        assert!(debug_str.contains("999"));
    }

    #[test]
    fn multiple_instances_are_independent() {
        let state1 = BucketState::new(5, 100);
        let state2 = BucketState::new(8, 200);

        assert_eq!(state1.current_tokens, 5);
        assert_eq!(state1.last_refill_timestamp, 100);

        assert_eq!(state2.current_tokens, 8);
        assert_eq!(state2.last_refill_timestamp, 200);
    }
}

#[cfg(test)]
mod storage_key_generations_tests {
    use super::*;

    fn make(service: &str, key: RateLimiterInputKey) -> RateLimiterInputData {
        RateLimiterInputData {
            service_name: service.to_string(),
            key,
        }
    }

    #[test]
    fn convert_to_storage_key_ip_variant() {
        let data = make("billing", RateLimiterInputKey::Ip("127.0.0.1".into()));
        assert_eq!(
            data.convert_to_storage_key(),
            "ratelimit:billing:ip:127.0.0.1"
        );
    }

    #[test]
    fn convert_to_storage_key_user_id_variant() {
        let data = make("auth", RateLimiterInputKey::UserId("user-123".into()));
        assert_eq!(
            data.convert_to_storage_key(),
            "ratelimit:auth:userId:user-123"
        );
    }

    #[test]
    fn convert_to_storage_key_api_key_variant() {
        let data = make("gateway", RateLimiterInputKey::ApiKey("sk_live_abc".into()));
        assert_eq!(
            data.convert_to_storage_key(),
            "ratelimit:gateway:apiKey:sk_live_abc"
        );
    }

    #[test]
    fn convert_to_storage_key_uuid_variant() {
        let data = make(
            "orders",
            RateLimiterInputKey::UUID("550e8400-e29b-41d4-a716-446655440000".into()),
        );
        assert_eq!(
            data.convert_to_storage_key(),
            "ratelimit:orders:uuid:550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn convert_to_storage_key_empty_service_name() {
        let data = make("", RateLimiterInputKey::Ip("1.2.3.4".into()));
        assert_eq!(data.convert_to_storage_key(), "ratelimit::ip:1.2.3.4");
    }

    #[test]
    fn convert_to_storage_key_empty_key_payload() {
        let data = make("svc", RateLimiterInputKey::ApiKey("".into()));
        assert_eq!(data.convert_to_storage_key(), "ratelimit:svc:apiKey:");
    }

    #[test]
    fn convert_to_storage_key_preserves_special_characters() {
        // Important because keys are colon-delimited; we at least ensure the function
        // does not sanitize/alter input unexpectedly.
        let data = make(
            "svc name/with spaces",
            RateLimiterInputKey::UserId("id:with:colons/and spaces".into()),
        );

        assert_eq!(
            data.convert_to_storage_key(),
            "ratelimit:svc name/with spaces:userId:id:with:colons/and spaces"
        );
    }

    #[test]
    fn convert_to_storage_key_is_deterministic() {
        let data = make("svc", RateLimiterInputKey::UUID("abc".into()));
        let a = data.convert_to_storage_key();
        let b = data.convert_to_storage_key();
        assert_eq!(a, b);
    }
}
