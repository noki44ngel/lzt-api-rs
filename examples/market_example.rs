use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("LZT_API_TOKEN")
        .unwrap_or_else(|_| "your_token".to_string());

    let client = LolzteamClient::new(&token);

    println!("=== Market API ===\n");

    println!("Search items:");
    let items = client.market().category_all(Default::default()).await?;
    println!("  Total: {}", items.total_items);

    println!("\nSteam items:");
    let steam = client.market().category_steam(Default::default()).await?;
    println!("  Total: {}", steam.total_items);

    println!("\nTags:");
    let tags = client.market().tags_list(None).await?;
    println!("  Total: {}", tags.total);

    println!("\nStats:");
    let stats = client.market().stats_get().await?;
    println!("  Items: {}, Sold: {}, Views: {}", stats.total_items, stats.total_sold, stats.total_views);

    Ok(())
}
