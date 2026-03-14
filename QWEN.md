# LZT API - Project Context

## Project Overview

**LZT API** is a production-ready Rust client library for Lolzteam Forum and Market services. It provides type-safe, async-ready API bindings that are **automatically generated** from OpenAPI 3.x schemas.

### Key Features

- **Auto-generated code** from OpenAPI schemas (`forum.json`, `market.json`)
- **Async/await** support with Tokio runtime
- **Automatic retry** with exponential backoff (429, 502, 503, 504)
- **HTTP/SOCKS proxy** support with authentication
- **Strong typing** with Serde serialization
- **Unified error handling** via `ApiError` enum
- **Builder pattern** for flexible client configuration
- **Full CRUD operations** (GET/POST/PUT/DELETE)

### Architecture

```
Api/
├── Cargo.toml              # Package manifest
├── build.rs                # Build script for code generation
├── forum.json              # OpenAPI Forum schema (26K lines)
├── market.json             # OpenAPI Market schema (65K lines)
├── LICENSE                 # MIT License
├── README.md               # Main documentation
├── ADAPTIVE_RETRY.md       # Retry & adaptation docs
├── IMPLEMENTATION_STATUS.md # API coverage details
├── run_tests.sh            # Integration test script
├── .github/workflows/
│   └── ci.yml              # GitHub Actions CI/CD
├── examples/               # Usage examples
│   ├── basic_usage.rs
│   ├── forum_example.rs
│   ├── market_example.rs
│   └── test_api.rs
└── src/
    ├── lib.rs              # Library root & re-exports
    ├── client.rs           # HTTP client + Builder pattern
    ├── error.rs            # ApiError enum
    ├── retry.rs            # Retry logic with backoff
    ├── proxy.rs            # Proxy configuration
    ├── codegen/
    │   └── main.rs         # CLI code generator
    └── generated/          # Auto-generated code
        ├── forum.rs        # Forum API client (29 methods)
        ├── market.rs       # Market API client (15 methods)
        ├── models.rs       # Common data models
        └── mod.rs          # Module exports
```

## Building and Running

### Prerequisites

- Rust 1.70+ (stable toolchain)
- Cargo package manager

### Build Commands

```bash
# Standard build
cargo build

# Release build (optimized)
cargo build --release

# Check compilation (fast)
cargo check

# Clean and rebuild
cargo clean && cargo build
```

### Running Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Forum API example (requires token)
export LZT_API_TOKEN="your_token"
cargo run --example forum_example

# Market API example (requires token)
cargo run --example market_example

# Integration tests (requires token)
cargo run --example test_api
```

### Testing

```bash
# Run unit tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests (requires API token)
./run_tests.sh YOUR_TOKEN_HERE

# Or with environment variable
export LZT_API_TOKEN="your_token"
./run_tests.sh
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-targets

# Strict linting
cargo clippy --all-targets -- -D warnings
```

### Code Generation

Code is **automatically generated** during build from OpenAPI schemas:

```bash
# Automatic (happens during build)
cargo build

# CLI tool for manual generation
cargo run --bin codegen openapi.yaml

# Regenerate after schema changes
cargo clean && cargo build
```

**Triggers for regeneration:**
- `forum.json` modified
- `market.json` modified
- Clean build (`cargo clean`)

## Development Conventions

### Code Style

- **Rust idioms**: Follow standard Rust conventions
- **Builder pattern**: Used for client configuration
- **Async/await**: All API methods are async
- **No unwrap**: Library code uses `Result<T, ApiError>`
- **Strong typing**: All structs derive `Serialize, Deserialize, Debug, Clone`

### Error Handling

All errors are handled via the unified `ApiError` enum:

```rust
pub enum ApiError {
    Network(reqwest::Error),           // Network errors
    Http { status, message },          // HTTP errors
    Deserialization(serde_json::Error), // JSON parsing
    RateLimit { retry_after: u64 },    // 429 with retry-after
    Unauthorized,                       // 401
    Forbidden(String),                  // 403
    NotFound(String),                   // 404
    ServerError { status, message },   // 5xx
    RetryLimitExceeded { attempts },   // All retries used
    // ... more variants
}
```

### Testing Practices

- **Unit tests**: In each module (`#[cfg(test)]`)
- **Integration tests**: In `examples/test_api.rs`
- **Test token**: Set via `LZT_API_TOKEN` environment variable
- **CI testing**: GitHub Actions runs all tests on push

### API Coverage

| API | Methods | Coverage |
|-----|---------|----------|
| Forum | 29 | Categories, Forums, Threads, Posts, Users, Conversations |
| Market | 15 | Search, Items, Tags, Stats |

### Retry Configuration

Automatic retry for: **429, 502, 503, 504**

```rust
use lzt_api::retry::RetryConfig;

let config = RetryConfig::new(
    5,              // max retries
    100,            // initial delay (ms)
    5000,           // max delay (ms)
)
.with_multiplier(2.0);  // exponential backoff
```

### Proxy Support

```rust
// HTTP proxy
.proxy("http://127.0.0.1:8080")

// With authentication
.proxy_with_auth("http://127.0.0.1:8080", "user", "pass")

// SOCKS5 proxy
.proxy("socks5://127.0.0.1:1080")
```

## GitHub Actions CI/CD

The CI pipeline (`.github/workflows/ci.yml`) runs on:
- Push to `main`
- Pull requests
- Release publication

**Jobs:**
1. **test**: fmt, clippy, unit tests, release build
2. **publish**: Publishes to crates.io on release (requires secret)

### Required Secrets

- `CARGO_REGISTRY_TOKEN`: crates.io API token (for publishing)

## Key Files Reference

| File | Purpose |
|------|---------|
| `Cargo.toml` | Dependencies and package metadata |
| `build.rs` | Build script for code generation |
| `src/lib.rs` | Library root, re-exports constants |
| `src/client.rs` | HTTP client with builder pattern |
| `src/error.rs` | Unified error types |
| `src/retry.rs` | Retry logic with exponential backoff |
| `src/proxy.rs` | Proxy configuration |
| `src/generated/*.rs` | Auto-generated API clients |
| `examples/test_api.rs` | Integration tests |
| `run_tests.sh` | Test runner script |

## API Base URLs

### Forum
- Primary: `https://api.lolz.live`
- Alternative: `https://api.zelenka.guru`

### Market
- Primary: `https://api.lzt.market`
- Alternative: `https://prod-api.lzt.market`

## Rate Limits

- **Forum**: 300 req/min (0.2s delay)
- **Market**: 120 req/min (0.5s delay)
- **Market Search**: 20 req/min (3s delay)

Automatic retry handles 429 responses with exponential backoff.

## Repository

- **GitHub**: https://github.com/noki44ngel/lzt-api-rs
- **License**: MIT
