use limiter_storage::{redis, traits};
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

    let redis_client = redis::redis_storage::RedisStorage::new(redis_url).unwrap();

    let data = BucketData {
        server: "auth".to_string(),
        ip: "134.324.55.1.34".to_string(),
        key: "Key_1".to_string(),
    };

    let json_data = serde_json::to_string(&data).unwrap();

    traits::AsyncQueryDatabase::create(&redis_client, key.clone(), json_data, None).await;

    let read_db: Option<String> =
        traits::AsyncQueryDatabase::find(&redis_client, key.clone()).await;

    let x = read_db.unwrap();
    println!("final result =>> {}", serde_json::to_value(x).unwrap());

    // traits::AsyncQueryDatabase::<String>::delete_bucket(&redis_client, key.clone()).await;
}
