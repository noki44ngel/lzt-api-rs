# LZT API

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/noki44ngel/lzt-api-rs/workflows/CI/badge.svg)](https://github.com/noki44ngel/lzt-api-rs/actions)

Production-ready Rust API client for Lolzteam Forum and Market services with automatic code generation from OpenAPI schemas.

## Features

- Auto-generated code from OpenAPI 3.x schemas
- Async/await support with Tokio
- Automatic retry with exponential backoff (429, 502, 503)
- HTTP/SOCKS proxy support
- Strong typing with Serde
- Unified error handling
- Full CRUD operations (GET/POST/PUT/DELETE)

## Installation

```toml
[dependencies]
lzt-api = { git = "https://github.com/noki44ngel/lzt-api-rs.git" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use lzt_api::{ApiClient, ApiResult, ForumClient};

#[tokio::main]
async fn main() -> ApiResult<()> {
    let client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .token("YOUR_API_TOKEN")
        .retry(3)
        .build()?;

    let forum = ForumClient::new(client);
    let categories = forum.get_categories().await?;
    println!("Categories: {}", categories.categories_total);
    Ok(())
}
```

## Forum API

### GET Methods
```rust
let forum = ForumClient::new(client);

let categories = forum.get_categories().await?;
let forums = forum.get_forums().await?;
let threads = forum.get_threads(forum_id, Some(page)).await?;
let user = forum.get_user(user_id).await?;
let me = forum.get_me().await?;
```

### POST Methods
```rust
use lzt_api::forum::CreateThreadRequest;

let request = CreateThreadRequest {
    forum_id: 1,
    title: "New Thread".to_string(),
    message: "Thread content".to_string(),
};
let thread = forum.create_thread(&request).await?;
```

### PUT Methods
```rust
use lzt_api::forum::UpdateUserRequest;

let request = UpdateUserRequest {
    username: Some("new_name".to_string()),
    ..Default::default()
};
let user = forum.update_user(user_id, &request).await?;
```

### DELETE Methods
```rust
forum.delete_thread(thread_id).await?;
forum.delete_post(post_id).await?;
```

## Market API

### GET Methods
```rust
let market = MarketClient::new(client);

let items = market.search_items(Some(params)).await?;
let steam = market.search_steam(Some(params)).await?;
let item = market.get_item(item_id).await?;
let tags = market.get_tags().await?;
let stats = market.get_stats().await?;
```

### POST Methods
```rust
use lzt_api::market::{CreateItemRequest, BuyItemRequest};

// Create item
let create = CreateItemRequest {
    category_id: 1,
    title: "Item".to_string(),
    description: "Desc".to_string(),
    price: 1000,
    currency: "RUB".to_string(),
};
let item = market.create_item(&create).await?;

// Buy item
let buy = BuyItemRequest { item_id: 123 };
market.buy_item(&buy).await?;
```

### PUT Methods
```rust
use lzt_api::market::UpdateItemRequest;

let update = UpdateItemRequest {
    price: Some(1500),
    ..Default::default()
};
market.update_item(item_id, &update).await?;
```

### DELETE Methods
```rust
market.delete_item(item_id).await?;
```

## Proxy Support

```rust
let client = ApiClient::builder()
    .base_url("https://api.lolz.live")
    .token("YOUR_TOKEN")
    .proxy("http://127.0.0.1:8080")
    .build()?;

// With authentication
let client = ApiClient::builder()
    .proxy_with_auth("http://127.0.0.1:8080", "user", "pass")
    .build()?;
```

## Retry Configuration

```rust
use lzt_api::retry::RetryConfig;

let config = RetryConfig::new(5, 100, 5000)
    .with_multiplier(2.0);

let client = ApiClient::builder()
    .retry_config(config)
    .build()?;
```

## Testing

```bash
cargo test

# Integration tests
export LZT_API_TOKEN="your_token"
cargo run --example test_api
```

## License

MIT License
