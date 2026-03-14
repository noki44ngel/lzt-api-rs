use crate::error::{ApiError, ApiResult};
use reqwest::Proxy;
use url::Url;

/// Proxy configuration for HTTP client
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ProxyConfig {
    pub fn new(url: impl Into<String>) -> Self {
        ProxyConfig {
            url: url.into(),
            username: None,
            password: None,
        }
    }

    pub fn with_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Build reqwest Proxy from configuration
    pub fn build(&self) -> ApiResult<Proxy> {
        let parsed_url = Url::parse(&self.url)
            .map_err(|e| ApiError::InvalidUrl(format!("Invalid proxy URL: {}", e)))?;

        let proxy = match parsed_url.scheme() {
            "http" | "https" => Proxy::http(&self.url),
            "socks4" => Proxy::all(&self.url),
            "socks5" => Proxy::all(&self.url),
            _ => Proxy::http(&self.url),
        };

        let mut proxy = proxy.map_err(|e| ApiError::Configuration(format!("Invalid proxy: {}", e)))?;

        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            proxy = proxy.basic_auth(username, password);
        }

        Ok(proxy)
    }
}

/// Proxy type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProxyType {
    #[default]
    Http,
    Https,
    Socks4,
    Socks5,
}

impl ProxyType {
    pub fn scheme(&self) -> &'static str {
        match self {
            ProxyType::Http => "http",
            ProxyType::Https => "https",
            ProxyType::Socks4 => "socks4",
            ProxyType::Socks5 => "socks5",
        }
    }
}

/// Builder for proxy configuration
#[derive(Debug, Default)]
pub struct ProxyBuilder {
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
    proxy_type: ProxyType,
}

impl ProxyBuilder {
    pub fn new() -> Self {
        ProxyBuilder::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    pub fn proxy_type(mut self, proxy_type: ProxyType) -> Self {
        self.proxy_type = proxy_type;
        self
    }

    pub fn build(self) -> ApiResult<Option<ProxyConfig>> {
        match self.url {
            Some(url) => {
                let mut config = ProxyConfig::new(url);
                if let (Some(username), Some(password)) = (self.username, self.password) {
                    config = config.with_auth(username, password);
                }
                Ok(Some(config))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_config_new() {
        let config = ProxyConfig::new("http://127.0.0.1:8080");
        assert_eq!(config.url, "http://127.0.0.1:8080");
        assert!(config.username.is_none());
        assert!(config.password.is_none());
    }

    #[test]
    fn test_proxy_config_with_auth() {
        let config = ProxyConfig::new("http://127.0.0.1:8080")
            .with_auth("user", "pass");
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
    }

    #[test]
    fn test_proxy_type_scheme() {
        assert_eq!(ProxyType::Http.scheme(), "http");
        assert_eq!(ProxyType::Https.scheme(), "https");
        assert_eq!(ProxyType::Socks4.scheme(), "socks4");
        assert_eq!(ProxyType::Socks5.scheme(), "socks5");
    }
}
