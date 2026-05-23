use crate::algorithms::token_bucket::{self, TokenBucket};
use crate::models::BucketState;
use crate::utils;
use limiter_storage::store::QueryDatabase;
pub trait RateLimiter {
    fn allow<S>(&self, key: String, storage: &S) -> bool
    where
        S: QueryDatabase<String, BucketState>;

    fn message(&self) {
        println!("a request arrives !");
    }
    fn status(&self) -> String;
}

impl RateLimiter for TokenBucket {
    fn allow<S>(&self, key: String, storage: &S) -> bool
    where
        S: QueryDatabase<String, BucketState>,
    {
        let bucket = storage.find(&key);

        match bucket {
            Some(data) => check_request(key, data, storage),
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

fn check_request<S>(id: String, data: BucketState, storage: &S) -> bool
where
    S: QueryDatabase<String, BucketState>,
{
    let token_bucket_status = TokenBucket::new(100, 1);

    let total_left_token = TokenBucket::refill_logic(
        data.last_refill_timestamp,
        token_bucket_status.refill_rate,
        token_bucket_status.capacity,
    );

    let result = TokenBucket::allow_deny_request(total_left_token, 10);

    if result {
        let new_data: BucketState = BucketState {
            current_tokens: 23,
            last_refill_timestamp: utils::time::timestamp(),
        };
        //update db
        storage.update_bucket_status(id, new_data);
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
