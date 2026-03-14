use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string());

    let client = LolzteamClient::new(&token);

    println!("=== Market API ===\n");

    println!("Search items:");
    let items = client.market().category_all(Default::default()).await?;
    println!("  Response: {:?}", items);

    println!("\nSteam items:");
    let steam = client.market().category_steam(Default::default()).await?;
    println!("  Response: {:?}", steam);

    println!("\nFavorites:");
    let favs = client.market().list_favorites(Default::default()).await?;
    println!("  Response: {:?}", favs);

    Ok(())
}
