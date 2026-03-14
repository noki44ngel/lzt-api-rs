use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("LZT_API_TOKEN")
        .unwrap_or_else(|_| "your_token".to_string());

    let client = LolzteamClient::new(&token);

    println!("=== Forum API ===\n");

    println!("Categories:");
    let categories = client.forum().categories_list(None, None, None).await?;
    println!("  Total: {}", categories.categories_total);

    println!("\nForums:");
    let forums = client.forum().forums_list(None, None, None).await?;
    println!("  Total: {}", forums.forums_total);

    println!("\nUser:");
    let user = client.forum().users_get(1, None).await?;
    println!("  User ID: {}", user.user_id);

    Ok(())
}
