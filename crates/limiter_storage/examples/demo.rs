use limiter_storage::{redis, store::AsyncRedisQueryDatabase};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct BucketData {
    server: String,
    ip: String,
    key: String,
}

#[tokio::main]
async fn main() {
    let redis_url = "redis://localhost:6379";
    let key = "test_key_1".to_string();

    let connection_manager = redis::redis_storage::RedisStorage::new(redis_url)
        .await
        .unwrap();

    let data = BucketData {
        server: "auth".to_string(),
        ip: "134.324.55.1.34".to_string(),
        key: "Key_1".to_string(),
    };

    let json_data = serde_json::to_string(&data).unwrap();

    let _ = connection_manager
        .create(key.clone(), json_data, None)
        .await
        .unwrap();

    let find_kye: String = connection_manager.find(key).await.unwrap();

    println!("result of key is {}", find_kye);
}
