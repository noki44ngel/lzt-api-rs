use lzt_api::{ApiClient, ApiResult, ForumClient};

#[tokio::main]
async fn main() -> ApiResult<()> {
    env_logger::init();
    let token = std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string());
    let client = ApiClient::builder().base_url("https://api.lolz.live").token(&token).retry(3).build()?;
    let forum = ForumClient::new(client);

    println!("=== Forum API ===\n");
    
    println!("Categories:");
    match forum.get_categories().await {
        Ok(r) => println!("  Total: {}", r.categories_total),
        Err(e) => eprintln!("  Error: {}", e),
    }

    println!("\nForums:");
    match forum.get_forums().await {
        Ok(r) => println!("  Total: {}", r.forums_total),
        Err(e) => eprintln!("  Error: {}", e),
    }

    println!("\nCurrent user:");
    match forum.get_me().await {
        Ok(r) => println!("  User: {:?}", r.user.get("username").unwrap_or(&serde_json::Value::String("unknown".to_string()))),
        Err(e) => eprintln!("  Error: {}", e),
    }

    Ok(())
}
