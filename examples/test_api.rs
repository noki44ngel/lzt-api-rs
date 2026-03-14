use lzt_api::{ApiClient, ApiError, ApiResult, ForumClient, MarketClient};
use lzt_api::market::SearchParams;

#[tokio::main]
async fn main() -> ApiResult<()> {
    env_logger::init();
    let token = std::env::var("LZT_API_TOKEN").expect("LZT_API_TOKEN must be set");
    println!("=== LZT API Integration Tests ===\n");

    let forum_client = ApiClient::builder().base_url("https://api.lolz.live").token(&token).retry(3).timeout(std::time::Duration::from_secs(30)).build()?;
    let forum = ForumClient::new(forum_client);

    println!("Test 1: Forum categories...");
    match forum.get_categories().await { Ok(r) => println!("  ✓ Success: {} categories", r.categories_total), Err(e) => println!("  ✗ Error: {}", e) }

    println!("Test 2: Forums...");
    match forum.get_forums().await { Ok(r) => println!("  ✓ Success: {} forums", r.forums_total), Err(e) => println!("  ✗ Error: {}", e) }

    println!("Test 3: Current user...");
    match forum.get_me().await { Ok(r) => println!("  ✓ Success: user fetched"), Err(e) => println!("  ✗ Error: {}", e) }

    let market_client = ApiClient::builder().base_url("https://api.lzt.market").token(&token).retry(3).timeout(std::time::Duration::from_secs(30)).build()?;
    let market = MarketClient::new(market_client);

    println!("\nTest 4: Search items...");
    let params = SearchParams { page: Some(1), pmin: Some(100.0), pmax: Some(1000.0), ..Default::default() };
    match market.search_items(Some(params)).await { Ok(r) => println!("  ✓ Success: {} items", r.total_items), Err(e) => println!("  ✗ Error: {}", e) }

    println!("Test 5: Steam items...");
    match market.search_steam(None).await { Ok(r) => println!("  ✓ Success: {} items", r.total_items), Err(e) => println!("  ✗ Error: {}", e) }

    println!("\nTest 6: Retry logic...");
    for i in 1..=3 { match forum.get_categories().await { Ok(_) => println!("  Request {}: OK", i), Err(ApiError::RateLimit { retry_after }) => println!("  Request {}: Rate limited ({}s)", i, retry_after), Err(e) => println!("  Request {}: {}", i, e) } }

    println!("\nTest 7: Unauthorized request...");
    let no_auth = ApiClient::builder().base_url("https://api.lolz.live").build()?;
    let no_auth_forum = ForumClient::new(no_auth);
    match no_auth_forum.get_categories().await { Ok(_) => println!("  ✗ Should be unauthorized"), Err(ApiError::Unauthorized) => println!("  ✓ Correctly returned Unauthorized"), Err(e) => println!("  ✓ Error: {}", e) }

    println!("\n=== All Tests Completed ===");
    Ok(())
}
