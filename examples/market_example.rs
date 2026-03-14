use lzt_api::{ApiClient, ApiResult, MarketClient};
use lzt_api::market::SearchParams;

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

    println!("=== Market API Demo ===\n");

    println!("Searching items...");
    let params = SearchParams {
        page: Some(1),
        pmin: Some(100.0),
        pmax: Some(1000.0),
        title: Some("steam".to_string()),
        ..Default::default()
    };

    match market.search_items(Some(params)).await {
        Ok(response) => {
            println!("Total items: {} (page {})", response.total_items, response.page);
            for item in response.items.iter().take(5) {
                println!("  - {} - {} RUB (views: {})", item.title, item.price, item.view_count);
            }
        }
        Err(e) => eprintln!("Error searching items: {}", e),
    }

    println!("\nSearching Steam items...");
    match market.search_steam(None).await {
        Ok(response) => println!("Total Steam items: {}", response.total_items),
        Err(e) => eprintln!("Error searching Steam items: {}", e),
    }

    println!("\nFetching tags...");
    match market.get_tags().await {
        Ok(response) => {
            println!("Total tags: {}", response.total);
            for tag in response.tags.iter().take(10) {
                println!("  - {} (ID: {})", tag.name, tag.tag_id);
            }
        }
        Err(e) => eprintln!("Error fetching tags: {}", e),
    }

    println!("\nFetching market statistics...");
    match market.get_stats().await {
        Ok(stats) => {
            println!("Total items: {}", stats.total_items);
            println!("Total sold: {}", stats.total_sold);
            println!("Total views: {}", stats.total_views);
        }
        Err(e) => eprintln!("Error fetching statistics: {}", e),
    }

    Ok(())
}
