# limiter-rs

Distributed rate limiter built with a token-bucket strategy in Rust.

## Overview

This workspace contains two crates:

- `limiter_core`: token-bucket algorithm and limiter trait.
- `limiter_storage`: pluggable storage backends (in-memory and Redis).

Use `limiter_core` in your app to perform checks, and configure storage from `limiter_storage`.

## Crates

### `limiter_core`

- `algorithms::token_bucket::TokenBucket`: token-bucket implementation.
- `limiter::RateLimiter`: trait with sync and async rate-limit checks.
- `models::bucket_state::BucketState`: stored bucket state (`current_tokens`, `last_refill_timestamp`).
- `models::key::{RateLimiterInputData, RateLimiterInputKey}`: helpers for consistent request keys.
- `policy::{RatelimiterConfig, StorageConfig, StorageType, AppConfig}`: config structs.

### `limiter_storage`

- `memory::memory_store::MemoryStore`: thread-safe in-memory store.
- `redis::redis_storage::RedisStorage`: Redis-backed async storage.
- `store::{SyncMemoryQueryDatabase, AsyncRedisQueryDatabase}`: storage traits used by limiter logic.

## Architecture

`TokenBucket` contains capacity and refill rate. For each request:

1. Fetch bucket state from storage by key.
2. Refill tokens based on elapsed time.
3. Allow/deny based on remaining tokens.
4. If allowed, persist updated bucket state.

## Installation

### Local workspace usage

From the workspace root:

```bash
git clone <repo-url>
cd limiter-rs
```

In a workspace root `Cargo.toml`:

```toml
[dependencies]
limiter_core = { path = "crates/limiter_core" }
limiter_storage = { path = "crates/limiter_storage" }
```

## Quick start (sync, in-memory)

```rust
use limiter_core::{
    algorithms::token_bucket::TokenBucket,
    limiter::RateLimiter,
    models::{bucket_state::BucketState, key::RateLimiterInputKey},
};
use limiter_core::models::key::RateLimiterInputData;
use limiter_storage::memory::memory_store::MemoryStore;

fn main() {
    let bucket = TokenBucket::new(100, 1);
    let mut storage: MemoryStore<String, BucketState> = MemoryStore::new();

    let key = RateLimiterInputData {
        service_name: "auth".to_string(),
        key: RateLimiterInputKey::Ip("127.0.0.1".to_string()),
    }
    .convert_to_storage_key();

    let allowed = bucket.check_rate(key, 5, &storage);
    println!("request allowed: {}", allowed);
}
```

## Quick start (async, Redis)

```rust
use limiter_core::{
    algorithms::token_bucket::TokenBucket,
    limiter::RateLimiter,
    models::{bucket_state::BucketState, key::RateLimiterInputKey},
};
use limiter_core::models::key::RateLimiterInputData;
use limiter_storage::redis::redis_storage::RedisStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bucket = TokenBucket::new(100, 1);
    let storage = RedisStorage::new("redis://127.0.0.1:6379").await?;

    let key = RateLimiterInputData {
        service_name: "auth".to_string(),
        key: RateLimiterInputKey::Ip("127.0.0.1".to_string()),
    }
    .convert_to_storage_key();

    let allowed = bucket.async_check_rate(key, 1, &storage).await;
    println!("request allowed: {}", allowed);
    Ok(())
}
```

## Configuration structs

- `RatelimiterConfig`: capacity, refill rate, and bucket TTL.
- `StorageConfig`: storage type (`Memory` or `Redis`) and optional `redis_url`.
- `AppConfig`: root config object that combines both.

You can use them in your own config loader and pass values into
`TokenBucket::new(...)` and storage initialization.

## Build and test

```bash
# run all workspace tests
cargo test

# run core demo example
cargo run --example demo -p limiter_core
```

## Storage notes

- `MemoryStore` stores values in-memory and uses optional TTL checks on read.
- `RedisStorage` persists serialized bucket state and uses a default TTL of 3600 seconds when no expiration is provided.

## License

MIT (see `LICENSE`).
