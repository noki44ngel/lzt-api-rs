use lzt_api::{ApiClient, ApiResult};

#[tokio::main]
async fn main() -> ApiResult<()> {
    env_logger::init();

    let client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .token(std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string()))
        .retry(3)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    println!("Client initialized: {}", client.base_url());

    Ok(())
}
