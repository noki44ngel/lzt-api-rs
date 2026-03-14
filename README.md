# LZT API

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/noki44ngel/lzt-api-rs/workflows/CI/badge.svg)](https://github.com/noki44ngel/lzt-api-rs/actions)

Production-ready Rust API client for Lolzteam Forum and Market services with **266 auto-generated endpoints**.

## Features

- ✅ **266 endpoints** - Forum (151) + Market (115)
- ✅ **Auto-generated** from OpenAPI schemas
- ✅ **Proxy support** - HTTP / HTTPS / SOCKS5
- ✅ **Auto-retry** - 429, 502, 503 with exponential backoff
- ✅ **Typed responses** - Serde serialization
- ✅ **MIT License** - Ready for publication

## Installation

```toml
[dependencies]
lzt-api = { git = "https://github.com/noki44ngel/lzt-api-rs.git" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LolzteamClient::new("YOUR_TOKEN");

    // Forum API
    let categories = client.forum().categories_list(None, None, None).await?;
    println!("Categories: {}", categories.categories_total);

    // Market API
    let items = client.market().category_all(Default::default()).await?;
    println!("Items: {}", items.total_items);

    Ok(())
}
```

## Authentication

Pass token via constructor or builder:

```rust
// Simple
let client = LolzteamClient::new("YOUR_TOKEN");

// With options
let client = LolzteamClient::builder("YOUR_TOKEN")
    .max_retries(3)
    .timeout(Duration::from_secs(30))
    .build()?;
```

## Proxy Support

```rust
// Single proxy
let client = LolzteamClient::builder("YOUR_TOKEN")
    .proxy("socks5://127.0.0.1:1080")
    .build()?;

// Separate proxies for Forum and Market
let client = LolzteamClient::builder("YOUR_TOKEN")
    .forum_proxy("socks5://forum-proxy:1080")
    .market_proxy("http://market-proxy:8080")
    .build()?;
```

## Retry Configuration

Automatic retry on: **429, 502, 503**

```rust
let client = LolzteamClient::builder("YOUR_TOKEN")
    .max_retries(5)           // Default: 5
    .timeout(Duration::from_secs(30))
    .build()?;
```

Exponential backoff: starts at 2s, max 60s.

## API Coverage

### Forum API (151 endpoints)

| Category | Endpoints |
|----------|-----------|
| Assets | CSS, batch execute |
| Categories | List, Get |
| Forums | List, Get, Follow, Unfollow, Followed |
| Threads | List, Get, Create, Edit, Delete, Move, Bump, Star, Follow |
| Posts | List, Get, Create, Edit, Delete, Like, Report |
| Users | Get, Me, Follow, Unfollow |
| Conversations | List, Get, Create, Delete, Invite, Kick |
| Chatbox | Messages, Leaderboard, Online, Ignore, Report |
| Tags | List, Get, Find, Popular |
| Navigation | List |
| Links | List, Get |
| Pages | List, Get |
| Polls | Get, Vote |
| Reports | Reasons, Create |
| Search | Forum, Thread, Post |
| ... | +100 more |

### Market API (115 endpoints)

| Category | Endpoints |
|----------|-----------|
| Category Search | All, Steam, Origin, Epic, Uplay, Battlenet, SocialClub, EAApp |
| Item Management | Get, Create, Edit, Delete, Close, Open |
| Lists | Favorites, Viewed, Orders, States, User, Download |
| Tags | List, Get, Add, Remove |
| Stats | Get |
| Profile | Get, Edit |
| Comments | List, Create, Delete |
| Reviews | List, Create, Delete |
| Discounts | List, Request, Cancel |
| Publishing | Add, Check, External, Fast Sell |
| Purchasing | Check, Confirm, Fast Buy |
| Managing | Bump, Edit, Delete, Guarantee, Claims |
| ... | +80 more |

## Project Structure

```
lzt-api/
├── Cargo.toml              # Package manifest
├── build.rs                # Build script for code generation
├── testnet.txt             # Test token (gitignored in production)
├── examples/
│   └── test_api.rs         # Integration tests
└── src/
    ├── lib.rs              # Library root
    ├── client.rs           # HTTP client with retry/proxy
    ├── error.rs            # Error types
    ├── models.rs           # Generated data models
    ├── forum/              # Forum API (151 endpoints)
    │   ├── methods.rs      # ← GENERATED: All Forum methods
    │   ├── types.rs        # ← GENERATED: All Forum types
    │   └── mod.rs          # Forum module
    └── market/             # Market API (115 endpoints)
        ├── methods.rs      # ← GENERATED: All Market methods
        ├── types.rs        # ← GENERATED: All Market types
        └── mod.rs          # Market module
```

## Code Generation

Code is **automatically generated** from OpenAPI schemas during build.

### Generated Files Location

```
src/
├── forum/methods.rs    # 151 Forum API methods
├── forum/types.rs      # Forum data types
├── market/methods.rs   # 115 Market API methods
└── market/types.rs     # Market data types
```

### Regenerate Code

```bash
# Clean and rebuild (triggers regeneration)
cargo clean && cargo build

# Or just build
cargo build
```

The `build.rs` script automatically:
1. Reads `forum.json` and `market.json` OpenAPI schemas
2. Generates typed Rust methods for all 266 endpoints
3. Generates data models with Serde serialization
4. Places generated code in `src/forum/` and `src/market/`

## Testing

```bash
# Set token
export LZT_API_TOKEN="your_token"
# Or create testnet.txt file

# Run integration tests
cargo run --example test_api --release

# Run unit tests
cargo test

# Run Clippy
cargo clippy --all-targets -- -D warnings

# Check formatting
cargo fmt --check
```

## Error Handling

```rust
use lzt_api::LolzteamClient;
use lzt_api::error::Error;

async fn fetch_data() -> Result<(), Error> {
    let client = LolzteamClient::new("YOUR_TOKEN");

    match client.forum().categories_list(None, None, None).await {
        Ok(response) => println!("Success: {}", response.categories_total),
        Err(Error::Unauthorized) => eprintln!("Authentication required"),
        Err(Error::RateLimit { retry_after }) => {
            eprintln!("Rate limited, retry after {}s", retry_after);
        }
        Err(Error::RetryLimitExceeded) => {
            eprintln!("Retry limit exceeded");
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```

## Base URLs

### Forum API
- Primary: `https://prod-api.lolz.live`
- Alternative: `https://prod-api.zelenka.guru`
- Fallback: `https://api.lolz.live`

### Market API
- Primary: `https://prod-api.lzt.market`
- Alternative: `https://api.lzt.market`

## Rate Limits

| API | Limit | Delay |
|-----|-------|-------|
| Forum | 300 req/min | 0.2s |
| Market | 120 req/min | 0.5s |
| Market Search | 20 req/min | 3s |

Automatic retry handles 429 responses.

## CI/CD

GitHub Actions runs on every push/PR:

- ✅ Code formatting (`cargo fmt --check`)
- ✅ Linting (`cargo clippy -- -D warnings`)
- ✅ Tests (`cargo test`)
- ✅ Build (`cargo build --release`)
- ✅ Publish to crates.io (on release tag)

### Configure Publishing

1. Get token from https://crates.io/settings/tokens
2. Add secret `CARGO_REGISTRY_TOKEN` in GitHub repo settings
3. Create release with tag `v*` (e.g., `v1.0.0`)

## License

MIT License - see [LICENSE](LICENSE) file.

## Links

- [Forum API Docs](https://lolzteam.readme.io/)
- [Market API Docs](https://lzt-market.readme.io/)
- [Telegram API Chat](https://t.me/lztmarket_api)
