pub enum RateLimiterInputKey {
    Ip(String),
    UserId(String),
    ApiKey(String),
    UUID(String),
}

pub struct RateLimiterInputData {
    pub service_name: String,
    pub key: RateLimiterInputKey,
}

impl RateLimiterInputData {
    pub fn convert_to_storage_key(&self) -> String {
        let key_part = match &self.key {
            RateLimiterInputKey::Ip(ip) => format!("ip:{}", ip),
            RateLimiterInputKey::UserId(id) => format!("userId:{}", id),
            RateLimiterInputKey::ApiKey(api_key) => format!("apiKey:{}", api_key),
            RateLimiterInputKey::UUID(uuid) => format!("uuid:{}", uuid),
        };

        format!("ratelimit:{}:{}", self.service_name, key_part)
    }
}
