use crate::algorithms::token_bucket::TokenBucket;
use crate::models::bucket_state::BucketState;
use crate::types::CheckRequestInput;

use limiter_storage::store::{AsyncRedisQueryDatabase, SyncMemoryQueryDatabase};

use crate::limiter_helper::{
    async_check_request, async_create_new_consumer, check_request, create_new_consumer,
};

pub trait RateLimiter {
    fn check_rate<S>(&self, key: String, consume: u64, storage: &S) -> bool
    where
        S: SyncMemoryQueryDatabase<String, BucketState>;

    fn async_check_rate<S>(
        &self,
        key: String,
        consume: u64,
        storage: &S,
    ) -> impl Future<Output = bool> + Send
    where
        S: AsyncRedisQueryDatabase<BucketState> + Send + Sync;

    fn message(&self) {
        println!("a request arrives !");
    }
}

impl RateLimiter for TokenBucket {
    fn check_rate<S>(&self, key: String, consume: u64, storage: &S) -> bool
    where
        S: SyncMemoryQueryDatabase<String, BucketState>,
    {
        let bucket = storage.find(key.clone());

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
            None => create_new_consumer(key, self.capacity, consume, storage),
        }
    }

    fn async_check_rate<S>(
        &self,
        key: String,
        consume: u64,
        storage: &S,
    ) -> impl Future<Output = bool> + Send
    where
        S: AsyncRedisQueryDatabase<BucketState> + Send + Sync,
    {
        async move {
            let bucket = storage.find(key.clone()).await;

            match bucket {
                Some(data) => {
                    async_check_request(
                        CheckRequestInput {
                            id: key,
                            bucket_state: data,
                            refill_rate: self.refill_rate,
                            capacity: self.capacity,
                            consume,
                        },
                        storage,
                    )
                    .await
                }

                None => async_create_new_consumer(key, self.capacity, consume, storage).await,
            }
        }
    }
}
