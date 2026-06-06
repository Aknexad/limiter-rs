use redis;

pub fn client(url: String) -> Result<redis::Client, redis::RedisError> {
    redis::Client::open(url)
}

pub async fn connection(url: String) {
    match redis::Client::open(url) {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to connect to Redis: {e}");
                return;
            }
        },
        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return;
        }
    };
}
