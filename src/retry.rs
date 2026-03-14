use crate::error::{ApiError, ApiResult};
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration with exponential backoff
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    pub fn new(max_retries: u32, initial_delay_ms: u64, max_delay_ms: u64) -> Self {
        RetryConfig {
            max_retries,
            initial_delay: Duration::from_millis(initial_delay_ms),
            max_delay: Duration::from_millis(max_delay_ms),
            multiplier: 2.0,
        }
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }
}

/// Retry executor with exponential backoff
pub struct RetryExecutor {
    config: RetryConfig,
}

impl RetryExecutor {
    pub fn new(config: RetryConfig) -> Self {
        RetryExecutor { config }
    }

    pub fn with_defaults() -> Self {
        RetryExecutor::new(RetryConfig::default())
    }

    /// Execute operation with retry logic
    pub async fn execute<T, F, Fut>(&self, mut operation: F) -> ApiResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ApiResult<T>>,
    {
        let mut attempts = 0;
        let mut delay = self.config.initial_delay;

        loop {
            attempts += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempts >= self.config.max_retries {
                        return Err(ApiError::RetryLimitExceeded { attempts });
                    }

                    if !e.is_retryable() {
                        return Err(e);
                    }

                    // Use retry-after header if available
                    let wait_time = e.retry_after().map_or(delay, Duration::from_secs);

                    sleep(wait_time).await;

                    // Exponential backoff
                    delay = std::cmp::min(
                        Duration::from_secs_f64(delay.as_secs_f64() * self.config.multiplier),
                        self.config.max_delay,
                    );
                }
            }
        }
    }
}

/// Check if status code should trigger retry
pub fn should_retry(status: u16) -> bool {
    matches!(status, 429 | 502 | 503 | 504)
}

/// Calculate delay with exponential backoff
pub fn calculate_backoff(attempt: u32, base_delay: Duration, max_delay: Duration, multiplier: f64) -> Duration {
    let delay = base_delay.as_secs_f64() * multiplier.powi(attempt as i32);
    std::cmp::min(Duration::from_secs_f64(delay), max_delay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_retry() {
        assert!(should_retry(429));
        assert!(should_retry(502));
        assert!(should_retry(503));
        assert!(should_retry(504));
        assert!(!should_retry(200));
        assert!(!should_retry(400));
        assert!(!should_retry(401));
        assert!(!should_retry(403));
        assert!(!should_retry(404));
    }

    #[test]
    fn test_calculate_backoff() {
        let base = Duration::from_millis(100);
        let max = Duration::from_secs(30);
        let multiplier = 2.0;

        assert_eq!(calculate_backoff(0, base, max, multiplier), Duration::from_millis(100));
        assert_eq!(calculate_backoff(1, base, max, multiplier), Duration::from_millis(200));
        assert_eq!(calculate_backoff(2, base, max, multiplier), Duration::from_millis(400));
    }
}
