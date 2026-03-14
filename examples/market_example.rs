use lzt_api::market::SearchParams;
use lzt_api::{ApiClient, ApiResult, MarketClient};

#[tokio::main]
async fn main() -> ApiResult<()> {
    env_logger::init();
    let token = std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string());
    let client = ApiClient::builder()
        .base_url("https://api.lzt.market")
        .token(&token)
        .retry(3)
        .build()?;
    let market = MarketClient::new(client);

    println!("=== Market API ===\n");

    println!("Search items:");
    let params = SearchParams {
        page: Some(1),
        pmin: Some(100.0),
        pmax: Some(1000.0),
        ..Default::default()
    };
    match market.search_items(Some(params)).await {
        Ok(r) => println!("  Total: {}", r.total_items),
        Err(e) => eprintln!("  Error: {}", e),
    }

    println!("\nSteam items:");
    match market.search_steam(None).await {
        Ok(r) => println!("  Total: {}", r.total_items),
        Err(e) => eprintln!("  Error: {}", e),
    }

    println!("\nTags:");
    match market.get_tags().await {
        Ok(r) => println!("  Total: {}", r.total),
        Err(e) => eprintln!("  Error: {}", e),
    }

    println!("\nStats:");
    match market.get_stats().await {
        Ok(s) => println!(
            "  Items: {}, Sold: {}, Views: {}",
            s.total_items, s.total_sold, s.total_views
        ),
        Err(e) => eprintln!("  Error: {}", e),
    }

    Ok(())
}
