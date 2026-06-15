use crate::algorithms::token_bucket::TokenBucket;
use crate::models::bucket_state::BucketState;
use crate::types::CheckRequestInput;
use crate::utils;

use limiter_storage::store::{AsyncRedisQueryDatabase, SyncMemoryQueryDatabase};

// sync
pub fn check_request<S>(data: CheckRequestInput<BucketState>, storage: &S) -> bool
where
    S: SyncMemoryQueryDatabase<String, BucketState>,
{
    let replenish_token = TokenBucket::refill_logic(
        data.bucket_state.last_refill_timestamp,
        data.refill_rate,
        data.capacity,
    );

    let total_left_token = TokenBucket::maximum_available_token_for_request(
        data.bucket_state.current_tokens,
        replenish_token,
        data.capacity,
    );

    let result = TokenBucket::allow_deny_request(total_left_token, data.consume);

    if result {
        let token_left_after_request =
            TokenBucket::reminder_token_after_request(data.consume, total_left_token);

        let new_data: BucketState = BucketState {
            current_tokens: token_left_after_request,
            last_refill_timestamp: utils::time::timestamp(),
        };
        //update db
        storage.update_bucket_status(data.id, new_data, None);
        return result;
    } else {
        return result;
    }
}

pub fn create_new_consumer<S>(id: String, capacity: u64, consume: u64, storage: &S) -> bool
where
    S: SyncMemoryQueryDatabase<String, BucketState>,
{
    let current_tokens = capacity.saturating_sub(consume);
    let allowed = consume <= capacity;
    let new_bucket = BucketState::new(current_tokens, utils::time::timestamp());
    println!("create new bucket for user with id {}", &id);
    storage.create(id, new_bucket, None);

    allowed
}

// Async

pub async fn async_check_request<S>(data: CheckRequestInput<BucketState>, storage: &S) -> bool
where
    S: AsyncRedisQueryDatabase<BucketState>,
{
    let replenish_token = TokenBucket::refill_logic(
        data.bucket_state.last_refill_timestamp,
        data.refill_rate,
        data.capacity,
    );

    let total_left_token = TokenBucket::maximum_available_token_for_request(
        data.bucket_state.current_tokens,
        replenish_token,
        data.capacity,
    );

    let result = TokenBucket::allow_deny_request(total_left_token, data.consume);

    if result {
        let token_left_after_request =
            TokenBucket::reminder_token_after_request(total_left_token, data.consume);

        let new_data: BucketState = BucketState {
            current_tokens: token_left_after_request,
            last_refill_timestamp: utils::time::timestamp(),
        };
        //update db
        storage
            .update_bucket_status(data.id, new_data, None)
            .await
            .unwrap();
        println!("update bucket status");
        return result;
    } else {
        return result;
    }
}

pub async fn async_create_new_consumer<S>(
    id: String,
    capacity: u64,
    consume: u64,
    storage: &S,
) -> bool
where
    S: AsyncRedisQueryDatabase<BucketState>,
{
    let current_tokens = capacity.saturating_sub(consume);
    let allowed = consume <= capacity;
    let new_bucket = BucketState::new(current_tokens, utils::time::timestamp());
    println!("create new bucket for user with id {}", &id);
    storage.create(id, new_bucket, None).await.unwrap();

    allowed
}
