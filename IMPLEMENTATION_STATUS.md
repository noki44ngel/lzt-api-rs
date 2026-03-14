# LZT API - Implementation Status

## Overview

This library provides **auto-generated** Rust bindings for Lolzteam Forum and Market APIs.

**OpenAPI Schemas:**
- `forum.json`: 26,523 lines, 151 unique operations
- `market.json`: 65,862 lines, 115 unique operations

## Implemented Endpoints

### Forum API (17 methods)

#### GET (11)
- ✅ `/categories` - List all categories
- ✅ `/categories/{id}` - Get category details
- ✅ `/forums` - List all forums
- ✅ `/forums/{id}` - Get forum details
- ✅ `/forums/{id}/threads` - List threads
- ✅ `/threads/{id}` - Get thread details
- ✅ `/threads/{id}/posts` - List posts
- ✅ `/users/{id}` - Get user profile
- ✅ `/users/me` - Get current user
- ✅ `/conversations` - List conversations
- ✅ `/conversations/{id}` - Get conversation

#### POST (2)
- ✅ `/threads` - Create new thread
- ✅ `/posts` - Create new post

#### PUT (1)
- ✅ `/users/{id}` - Update user

#### DELETE (3)
- ✅ `/threads/{id}` - Delete thread
- ✅ `/posts/{id}` - Delete post
- ✅ `/conversations/{id}` - Delete conversation

### Market API (15 methods)

#### GET (11)
- ✅ `/` - Search all items
- ✅ `/{id}` - Get item details
- ✅ `/steam` - Search Steam items
- ✅ `/origin` - Search Origin items
- ✅ `/epic` - Search Epic Games items
- ✅ `/uplay` - Search Uplay items
- ✅ `/battlenet` - Search Battle.net items
- ✅ `/socialclub` - Search Social Club items
- ✅ `/eaapp` - Search EA App items
- ✅ `/tags` - Get all tags
- ✅ `/stats` - Get market statistics

#### POST (2)
- ✅ `/` - Create new item
- ✅ `/{id}` - Buy item

#### PUT (1)
- ✅ `/{id}` - Update item

#### DELETE (1)
- ✅ `/{id}` - Delete item

## Coverage

| API | Total Operations | Implemented | Coverage |
|-----|-----------------|-------------|----------|
| Forum | 151 | 17 | ~11% |
| Market | 115 | 15 | ~13% |

**Note:** The implemented endpoints cover the **most commonly used** operations. The remaining endpoints are variations and specialized operations that can be added using the same code generation pattern.

## Code Generation

The library uses `build.rs` to automatically generate code from OpenAPI schemas:

```bash
# Automatic generation during build
cargo build

# Manual generation
cargo run --bin codegen openapi.yaml
```

## Adding More Endpoints

To add additional endpoints:

1. **Manual approach**: Add methods to `src/generated/forum.rs` or `market.rs`
2. **Automatic approach**: Update `build.rs` to generate more endpoints from OpenAPI

Example - Adding a new endpoint:

```rust
// In src/generated/forum.rs
pub async fn get_thread_followers(&self, thread_id: i64) -> ApiResult<FollowersResponse> {
    let builder = self.api().get(&format!("/threads/{}/followers", thread_id));
    self.api().execute_json(builder).await
}
```

## Testing

```bash
# Set token
export LZT_API_TOKEN="your_token"

# Run tests
cargo test

# Run integration tests
./run_tests.sh

# Or directly
cargo run --example test_api
```

## Features

All implemented endpoints include:
- ✅ Automatic retry (429, 502, 503)
- ✅ Proxy support (HTTP/SOCKS)
- ✅ Error handling
- ✅ Type safety with Serde
- ✅ Async/await support

## Token Requirements

For full API access, your token needs these scopes:
- `basic` - Basic access
- `read` - Read operations
- `post` - Create/edit operations
- `market` - Market operations
- `conversate` - Conversations

Get your token from: https://lolz.live/account/api
