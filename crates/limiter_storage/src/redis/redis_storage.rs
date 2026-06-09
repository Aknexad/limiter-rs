use redis::AsyncCommands;

pub struct RedisStorage {
    pub client: redis::Client,
}

impl RedisStorage {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        Ok(Self {
            client: redis::Client::open(url)?,
        })
    }

    pub async fn connection(
        client: &redis::Client,
    ) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
        client.get_multiplexed_async_connection().await
    }
}

// pub fn client(url: String) -> Result<redis::Client, redis::RedisError> {
//     redis::Client::open(url)
// }

// pub async fn connection(
//     client: &redis::Client,
// ) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
//     client.get_multiplexed_async_connection().await
// }

// #[tokio::main]
// pub async fn test_call() {
//     let redis_url = "redis://localhost:6379".to_string();
//     let client = client(redis_url).expect("Failed to create Redis client");
//     let mut connection = connection(&client)
//         .await
//         .expect("Failed to connect to Redis");

//     let _: () = connection
//         .set("t1", "23")
//         .await
//         .expect("Failed to set value in Redis");

//     let value: String = connection
//         .get("t1")
//         .await
//         .expect("Failed to get value from Redis");
//     println!("Value for 't1': {}", value);
// }
