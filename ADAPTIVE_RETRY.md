# LZT API - Adaptive Code Generation & Auto-Retry

## 🔄 Automatic API Adaptation

### How It Works

The library **automatically adapts** to API changes through:

1. **Build-time Code Generation** (`build.rs`)
   - Monitors `forum.json` and `market.json` for changes
   - Automatically regenerates code when schemas are updated
   - Ensures type safety matches the current API

2. **OpenAPI-Driven Types**
   - All structs generated from OpenAPI schemas
   - Field types match API exactly
   - Missing/extra fields handled with `Option<T>` and `#[serde(flatten)]`

3. **Compile-time Safety**
   - API changes detected at compile time
   - Type mismatches caught early
   - No runtime surprises

### When Code Regenerates

```bash
# Automatic - when you build
cargo build

# Manual trigger
cargo clean && cargo build

# CLI tool
cargo run --bin codegen openapi.yaml
```

**Triggers:**
- ✅ `forum.json` modified
- ✅ `market.json` modified
- ✅ Clean build (`cargo clean`)

## ⚡ Automatic Retry System

### Configured Status Codes

The library **automatically retries** on these HTTP status codes:

| Code | Name | Auto-Retry | Backoff |
|------|------|------------|---------|
| 429 | Too Many Requests | ✅ Yes | Respects `Retry-After` header |
| 502 | Bad Gateway | ✅ Yes | Exponential |
| 503 | Service Unavailable | ✅ Yes | Exponential |
| 504 | Gateway Timeout | ✅ Yes | Exponential |

### Retry Configuration

```rust
use lzt_api::retry::RetryConfig;
use std::time::Duration;

// Default configuration
let default = RetryConfig::default();
// max_retries: 3
// initial_delay: 100ms
// max_delay: 30s
// multiplier: 2.0

// Custom configuration
let custom = RetryConfig::new(
    5,              // max retries
    100,            // initial delay (ms)
    5000,           // max delay (ms)
)
.with_multiplier(2.0);  // exponential backoff

let client = ApiClient::builder()
    .retry_config(custom)
    .build()?;
```

### How Retry Works

```
Request → 429/502/503 → Wait (100ms) → Retry 1
                     → 429/502/503 → Wait (200ms) → Retry 2
                                  → 429/502/503 → Wait (400ms) → Retry 3
                                               → Success! ✅
                                               → OR: RetryLimitExceeded ❌
```

**Backoff Formula:**
```
delay = min(initial_delay * (multiplier ^ attempt), max_delay)
```

### Example with Retry

```rust
use lzt_api::{ApiClient, ApiError, ApiResult};

async fn fetch_with_retry() -> ApiResult<()> {
    let client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .token("YOUR_TOKEN")
        .retry(5)  // Enable retry with 5 attempts
        .build()?;

    let forum = lzt_api::ForumClient::new(client);

    // This will automatically retry on 429/502/503/504
    match forum.get_categories().await {
        Ok(response) => {
            println!("Success: {} categories", response.categories_total);
        }
        Err(ApiError::RateLimit { retry_after }) => {
            // 429 with Retry-After header
            eprintln!("Rate limited, waiting {}s", retry_after);
        }
        Err(ApiError::RetryLimitExceeded { attempts }) => {
            // All retries exhausted
            eprintln!("Failed after {} attempts", attempts);
        }
        Err(e) => {
            // Other errors (not retryable)
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
```

### Retry-After Header Support

When the API returns `429 Too Many Requests` with a `Retry-After` header:

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 60
```

The library **automatically waits** the specified time before retrying.

### Non-Retryable Errors

These errors are **NOT** retried (fail immediately):

- ❌ 400 Bad Request
- ❌ 401 Unauthorized
- ❌ 403 Forbidden
- ❌ 404 Not Found
- ❌ Network errors (except timeout)
- ❌ Deserialization errors

### Testing Retry Logic

```rust
#[tokio::test]
async fn test_retry_on_rate_limit() {
    let client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .token("YOUR_TOKEN")
        .retry(3)
        .build()
        .unwrap();

    let forum = lzt_api::ForumClient::new(client);

    // Make rapid requests to trigger rate limit
    for i in 1..=10 {
        match forum.get_categories().await {
            Ok(_) => println!("Request {}: OK", i),
            Err(ApiError::RateLimit { retry_after }) => {
                println!("Request {}: Rate limited (waiting {}s)", i, retry_after);
            }
            Err(e) => println!("Request {}: {}", i, e),
        }
    }
}
```

## 🛡️ Error Handling

### ApiError Enum

```rust
pub enum ApiError {
    Network(reqwest::Error),           // Network errors
    Http { status, message },          // HTTP errors
    Deserialization(serde_json::Error), // JSON parsing
    RateLimit { retry_after: u64 },    // 429 with retry-after
    Unauthorized,                       // 401
    Forbidden(String),                  // 403
    NotFound(String),                   // 404
    BadRequest(String),                 // 400
    ServerError { status, message },   // 5xx
    RetryLimitExceeded { attempts: u32 }, // All retries used
    // ... more
}
```

### Check if Error is Retryable

```rust
impl ApiError {
    pub fn is_retryable(&self) -> bool {
        match self {
            ApiError::Http { status, .. } | ApiError::ServerError { status, .. } => {
                matches!(status.as_u16(), 429 | 502 | 503 | 504)
            }
            ApiError::Network(e) => e.is_timeout() || e.is_connect(),
            ApiError::Timeout => true,
            _ => false,
        }
    }

    pub fn retry_after(&self) -> Option<u64> {
        match self {
            ApiError::RateLimit { retry_after } => Some(*retry_after),
            ApiError::Http { status, .. } if status.as_u16() == 429 => Some(1),
            _ => None,
        }
    }
}
```

## 📊 Summary

| Feature | Status | Details |
|---------|--------|---------|
| Auto-regenerate on schema change | ✅ | `build.rs` monitors JSON files |
| Type-safe API bindings | ✅ | Generated from OpenAPI |
| Compile-time error detection | ✅ | Rust type system |
| Automatic retry (429) | ✅ | Respects Retry-After header |
| Automatic retry (502/503/504) | ✅ | Exponential backoff |
| Configurable retry | ✅ | max_retries, delays, multiplier |
| Non-retryable errors | ✅ | 400, 401, 403, 404 fail fast |

## 🔧 Maintenance

### Updating API Schemas

```bash
# 1. Download new schemas
curl -o forum.json https://api.lolz.live/openapi.json
curl -o market.json https://api.lzt.market/openapi.json

# 2. Rebuild (auto-regenerates code)
cargo clean && cargo build

# 3. Run tests
cargo test
```

### Checking Generated Code

```bash
# View generated Forum client
cat src/generated/forum.rs

# View generated Market client
cat src/generated/market.rs

# View generated models
cat src/generated/models.rs
```

**The library is fully adaptive and production-ready!** 🚀
