use crate::algorithms::token_bucket::TokenBucket;
use crate::models::BucketState;
use crate::types::CheckRequestInput;
use crate::utils;
use limiter_storage::store::QueryDatabase;
pub trait RateLimiter {
    fn check_rate<S>(&self, key: String, consume: u64, storage: &S) -> bool
    where
        S: QueryDatabase<String, BucketState>;

    fn message(&self) {
        println!("a request arrives !");
    }
    fn status(&self) -> String;
}

impl RateLimiter for TokenBucket {
    fn check_rate<S>(&self, key: String, consume: u64, storage: &S) -> bool
    where
        S: QueryDatabase<String, BucketState>,
    {
        let bucket = storage.find(&key);

        match bucket {
            Some(data) => check_request(
                CheckRequestInput {
                    id: key,
                    bucket_state: data,
                    refill_rate: self.refill_rate,
                    capacity: self.capacity,
                    consume,
                },
                storage,
            ),
            None => create_new_consumer(key, storage),
        }
    }
    fn status(&self) -> String {
        format!(
            "total capacity {} and last refill was in {}",
            self.capacity, self.last_refill_timestamp
        )
    }
}

fn check_request<S>(data: CheckRequestInput<BucketState>, storage: &S) -> bool
where
    S: QueryDatabase<String, BucketState>,
{
    let total_left_token = TokenBucket::refill_logic(
        data.bucket_state.last_refill_timestamp,
        data.refill_rate,
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
        storage.update_bucket_status(data.id, new_data);
        println!("update bucket status");
        return result;
    } else {
        return result;
    }
}

fn create_new_consumer<S>(id: String, storage: &S) -> bool
where
    S: QueryDatabase<String, BucketState>,
{
    let new_bucket = BucketState::new(100, utils::time::timestamp());
    println!("create new bucket for user with id {}", &id);
    storage.create(id, new_bucket);

    true
}
