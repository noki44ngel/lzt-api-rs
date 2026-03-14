use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string());

    let client = LolzteamClient::new(&token);

    let categories = client.forum().categories_list(None, None, None).await?;
    println!("Categories: {}", categories.categories_total);

    Ok(())
}
