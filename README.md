# limiter-rs

![Rust](https://img.shields.io/badge/rust-1.70%2B-000000.svg?logo=rust)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

`limiter-rs` is a lightweight, distributed-friendly rate-limiting library built in Rust.
It uses a **token bucket** algorithm and supports both in-memory and Redis-backed
storage for scalable request control.

## Table of Contents

- [Highlights](#highlights)
- [Repository Structure](#repository-structure)
- [Key Types](#key-types)
- [Core Concepts](#core-concepts)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Storage Backends](#storage-backends)
- [Testing](#testing)
- [License](#license)

## Highlights

- Token-bucket throttling with configurable capacity and refill rate.
- Sync and async API support via a shared algorithm.
- Deterministic storage-key generation (`Ip`, `UserId`, `ApiKey`, `UUID`).
- Extensible storage abstraction with pluggable implementations.
- Workspace layout suitable for reuse in microservices or shared infra crates.

## Repository Structure

- `crates/limiter_core`: algorithm, limiter trait, domain models, and policy types.
- `crates/limiter_storage`: storage traits and backend implementations.
- `tests/`: integration tests for end-to-end behavior.
- `crates/limiter_core/examples/demo.rs`: runnable usage example.

## Key Types

### `limiter_core`

- `algorithms::token_bucket::TokenBucket`
- `limiter::RateLimiter`
- `models::bucket_state::BucketState`
- `models::key::RateLimiterInputData`
- `models::key::RateLimiterInputKey`
- `policy::{RatelimiterConfig, StorageConfig, StorageType, AppConfig}`

### `limiter_storage`

- `memory::memory_store::MemoryStore`
- `redis::redis_storage::RedisStorage`
- `store::{SyncMemoryQueryDatabase, AsyncRedisQueryDatabase}`

## Core Concepts

1. A `TokenBucket` instance defines request budget:
   - `capacity`: maximum tokens in bucket.
   - `refill_rate`: tokens regenerated per second.
2. `check_rate` / `async_check_rate` computes current available tokens:
   - applies refill logic using the last update timestamp.
3. Request is allowed only if tokens are sufficient.
4. For allowed requests, state is updated in the selected storage backend.

## Quick Start

### Add dependencies

From the workspace root, use local paths while developing:

```toml
[dependencies]
limiter_core = { path = "crates/limiter_core" }
limiter_storage = { path = "crates/limiter_storage" }
```

### Sync example (in-memory)

```rust
use limiter_core::{
    algorithms::token_bucket::TokenBucket,
    limiter::RateLimiter,
    models::{bucket_state::BucketState, key::{RateLimiterInputData, RateLimiterInputKey}},
};
use limiter_storage::memory::memory_store::MemoryStore;

fn main() {
    let bucket = TokenBucket::new(100, 1);
    let storage: MemoryStore<String, BucketState> = MemoryStore::new();

    let key = RateLimiterInputData {
        service_name: "auth".to_string(),
        key: RateLimiterInputKey::Ip("127.0.0.1".to_string()),
    }
    .convert_to_storage_key();

    let allowed = bucket.check_rate(key, 5, &storage);
    println!("request allowed: {allowed}");
}
```

### Async example (Redis)

```rust
use limiter_core::{
    algorithms::token_bucket::TokenBucket,
    limiter::RateLimiter,
    models::{bucket_state::BucketState, key::{RateLimiterInputData, RateLimiterInputKey}},
};
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
    println!("request allowed: {allowed}");
    Ok(())
}
```

## Configuration

Use these config structs to initialize limiter and storage choices from a single
configuration source:

- `RatelimiterConfig`: token bucket limits and TTL settings.
- `StorageConfig`: choose backend (`Memory` or `Redis`) and optional Redis URL.
- `AppConfig`: combines rate limiter and storage config.

`limiter_storage` does not read config directly; pass constructed values into
your chosen backend initialization.

## Storage Backends

- `MemoryStore`
  - Thread-safe, in-process storage.
  - Suitable for single-process or test scenarios.
- `RedisStorage`
  - Async distributed storage using Redis.
  - Defaults to TTL of 3600 seconds when expiration is not provided.

## Testing

```bash
cargo test
```

To run the example:

```bash
cargo run --example demo -p limiter_core
```

## License

Licensed under the Apache License 2.0. See `LICENSE`.
