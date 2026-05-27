use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RatelimiterConfig {
    pub token_bucket_max_capacity: u64,
    pub token_bucket_refill_rate: u64,
    pub bucket_ttl_seconds: u64,
}

#[derive(Deserialize, Debug)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub redis_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum StorageType {
    Memory,
    Redis,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub rate_limit: RatelimiterConfig,
    pub storage: StorageConfig,
}
