use crate::error::{ApiError, ApiResult};
use crate::proxy::ProxyConfig;
use crate::retry::{RetryConfig, RetryExecutor};
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// API Client builder with fluent interface
#[derive(Debug, Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    token: Option<String>,
    proxy: Option<ProxyConfig>,
    retry_config: Option<RetryConfig>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        ClientBuilder::default()
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy = Some(ProxyConfig::new(proxy_url));
        self
    }

    pub fn proxy_with_auth(
        mut self,
        proxy_url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.proxy = Some(ProxyConfig::new(proxy_url).with_auth(username, password));
        self
    }

    pub fn retry(mut self, max_retries: u32) -> Self {
        self.retry_config = Some(RetryConfig::default().with_max_retries(max_retries));
        self
    }

    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    /// Build configured API client
    pub fn build(self) -> ApiResult<ApiClient> {
        let base_url = self
            .base_url
            .ok_or_else(|| ApiError::Configuration("Base URL is required".to_string()))?;

        let mut client_builder = Client::builder();

        if let Some(proxy_config) = self.proxy {
            let proxy = proxy_config.build()?;
            client_builder = client_builder.proxy(proxy);
        }

        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        let client = client_builder.build().map_err(ApiError::Network)?;

        Ok(ApiClient {
            client,
            base_url,
            token: self.token,
            retry_config: self.retry_config.unwrap_or_default(),
            user_agent: self.user_agent.unwrap_or_else(|| "lzt-api/1.0".to_string()),
        })
    }
}

/// Main API client for HTTP requests
#[derive(Debug)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
    retry_config: RetryConfig,
    user_agent: String,
}

impl ApiClient {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn new(base_url: impl Into<String>) -> Self {
        ApiClient {
            client: Client::new(),
            base_url: base_url.into(),
            token: None,
            retry_config: RetryConfig::default(),
            user_agent: "lzt-api/1.0".to_string(),
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get(&self, endpoint: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut builder = self.client.get(&url);
        builder = self.apply_headers(builder);
        builder
    }

    pub fn post(&self, endpoint: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut builder = self.client.post(&url);
        builder = self.apply_headers(builder);
        builder
    }

    pub fn put(&self, endpoint: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut builder = self.client.put(&url);
        builder = self.apply_headers(builder);
        builder
    }

    pub fn delete(&self, endpoint: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut builder = self.client.delete(&url);
        builder = self.apply_headers(builder);
        builder
    }

    /// Execute request with retry logic
    pub async fn execute(&self, builder: RequestBuilder) -> ApiResult<Response> {
        let retry_executor = RetryExecutor::new(self.retry_config.clone());

        retry_executor
            .execute(|| async {
                let request = builder.try_clone().ok_or_else(|| {
                    ApiError::Configuration("Request cannot be cloned".to_string())
                })?;

                let response = request.send().await.map_err(ApiError::from)?;
                self.handle_response(response).await
            })
            .await
    }

    /// Execute request and deserialize JSON response
    pub async fn execute_json<T: DeserializeOwned>(&self, builder: RequestBuilder) -> ApiResult<T> {
        let response = self.execute(builder).await?;
        let text = response.text().await.map_err(ApiError::from)?;
        serde_json::from_str(&text).map_err(ApiError::from)
    }

    /// Handle HTTP response and convert errors
    async fn handle_response(&self, response: Response) -> ApiResult<Response> {
        let status = response.status();

        match status.as_u16() {
            200..=299 => Ok(response),
            401 => Err(ApiError::Unauthorized),
            403 => Err(ApiError::Forbidden("Access denied".to_string())),
            404 => Err(ApiError::NotFound("Resource not found".to_string())),
            429 => {
                let retry_after = response
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(1);
                Err(ApiError::RateLimit { retry_after })
            }
            400..=499 => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Bad request".to_string());
                Err(ApiError::BadRequest(message))
            }
            500..=599 => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Server error".to_string());
                Err(ApiError::server_error(status, message))
            }
            _ => Err(ApiError::http(status, "Unknown error")),
        }
    }

    /// Apply common headers to request
    fn apply_headers(&self, mut builder: RequestBuilder) -> RequestBuilder {
        builder = builder.header("User-Agent", &self.user_agent);
        builder = builder.header("Accept", "application/json");
        builder = builder.header("Content-Type", "application/json");

        if let Some(token) = &self.token {
            builder = builder.header("Authorization", format!("Bearer {}", token));
        }

        builder
    }
}

/// Forum API client wrapper
#[derive(Debug)]
pub struct ForumClient {
    client: ApiClient,
}

impl ForumClient {
    pub fn new(client: ApiClient) -> Self {
        ForumClient { client }
    }

    pub fn api(&self) -> &ApiClient {
        &self.client
    }
}

/// Market API client wrapper
#[derive(Debug)]
pub struct MarketClient {
    client: ApiClient,
}

impl MarketClient {
    pub fn new(client: ApiClient) -> Self {
        MarketClient { client }
    }

    pub fn api(&self) -> &ApiClient {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let builder = ClientBuilder::new()
            .base_url("https://api.example.com")
            .token("test_token")
            .proxy("http://127.0.0.1:8080")
            .retry(3)
            .timeout(Duration::from_secs(30))
            .user_agent("test-agent/1.0");

        assert!(builder.base_url.is_some());
        assert!(builder.token.is_some());
        assert!(builder.proxy.is_some());
        assert!(builder.retry_config.is_some());
        assert!(builder.timeout.is_some());
        assert!(builder.user_agent.is_some());
    }

    #[test]
    fn test_client_new() {
        let client = ApiClient::new("https://api.example.com");
        assert_eq!(client.base_url(), "https://api.example.com");
    }
}
