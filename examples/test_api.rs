use lzt_api::{ApiClient, ApiError, ApiResult, ForumClient, MarketClient};
use lzt_api::market::SearchParams;

#[tokio::main]
async fn main() -> ApiResult<()> {
    env_logger::init();

    let token = std::env::var("LZT_API_TOKEN")
        .expect("LZT_API_TOKEN environment variable must be set");

    println!("=== LZT API Integration Tests ===\n");

    // Initialize Forum client
    let forum_client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .token(&token)
        .retry(3)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let forum = ForumClient::new(forum_client);

    // Test 1: Get categories
    println!("Test 1: Fetching forum categories...");
    match forum.get_categories().await {
        Ok(response) => println!("  ✓ Success: {} categories", response.categories_total),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    // Test 2: Get forums
    println!("Test 2: Fetching forums...");
    match forum.get_forums().await {
        Ok(response) => println!("  ✓ Success: {} forums", response.forums_total),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    // Test 3: Get current user
    println!("Test 3: Fetching current user...");
    match forum.get_me().await {
        Ok(response) => println!("  ✓ Success: {} (ID: {})", response.user.username, response.user.user_id),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    // Initialize Market client
    let market_client = ApiClient::builder()
        .base_url("https://api.lzt.market")
        .token(&token)
        .retry(3)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let market = MarketClient::new(market_client);

    // Test 4: Search items
    println!("\nTest 4: Searching items...");
    let params = SearchParams {
        page: Some(1),
        pmin: Some(100.0),
        pmax: Some(1000.0),
        ..Default::default()
    };
    match market.search_items(Some(params)).await {
        Ok(response) => println!("  ✓ Success: {} items (page {})", response.total_items, response.page),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    // Test 5: Search Steam items
    println!("Test 5: Searching Steam items...");
    match market.search_steam(None).await {
        Ok(response) => println!("  ✓ Success: {} Steam items", response.total_items),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    // Test 6: Retry logic test
    println!("\nTest 6: Testing retry logic (rapid requests)...");
    for i in 1..=3 {
        match forum.get_categories().await {
            Ok(_) => println!("  Request {}: OK", i),
            Err(ApiError::RateLimit { retry_after }) => {
                println!("  Request {}: Rate limited (waiting {}s)", i, retry_after);
            }
            Err(e) => println!("  Request {}: {}", i, e),
        }
    }

    // Test 7: Unauthorized request
    println!("\nTest 7: Testing unauthorized request...");
    let no_auth_client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .build()?;
    
    let no_auth_forum = ForumClient::new(no_auth_client);
    match no_auth_forum.get_categories().await {
        Ok(_) => println!("  ✗ Expected unauthorized error"),
        Err(ApiError::Unauthorized) => println!("  ✓ Success: Properly returned Unauthorized"),
        Err(e) => println!("  ✓ Success: {}", e),
    }

    println!("\n=== All Tests Completed ===");

    Ok(())
}
