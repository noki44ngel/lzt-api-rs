//! LZT API - Production-ready Rust client for Lolzteam Forum and Market
//!
//! Features:
//! - Auto-generated code from OpenAPI schemas
//! - Async/await with Tokio
//! - Automatic retry with exponential backoff
//! - HTTP/SOCKS proxy support
//! - Strong typing with Serde
//! - Unified error handling

pub mod client;
pub mod error;
pub mod proxy;
pub mod retry;

pub mod generated;

// Re-export main types
pub use client::{ApiClient, ClientBuilder, ForumClient, MarketClient};
pub use error::{ApiError, ApiResult};
pub use proxy::ProxyConfig;
pub use retry::RetryConfig;

// Re-export generated types
pub use generated::forum;
pub use generated::market;
pub use generated::models::*;

// Library constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_USER_AGENT: &str = "lzt-api/1.0";

// Forum API endpoints
pub const FORUM_BASE_URL: &str = "https://api.lolz.live";
pub const FORUM_BASE_URL_ALT: &str = "https://api.zelenka.guru";

// Market API endpoints
pub const MARKET_BASE_URL: &str = "https://api.lzt.market";
pub const MARKET_BASE_URL_ALT: &str = "https://prod-api.lzt.market";
