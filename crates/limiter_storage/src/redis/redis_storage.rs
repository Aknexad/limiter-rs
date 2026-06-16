use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct RedisStorage {
    pub connections_manager: ConnectionManager,
}

impl RedisStorage {
    pub async fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;

        let con_manager = ConnectionManager::new(client).await?;

        let result = Self {
            connections_manager: con_manager,
        };

        Ok(result)
    }
}
