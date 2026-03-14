use lzt_api::{ApiClient, ApiResult, ForumClient};

#[tokio::main]
async fn main() -> ApiResult<()> {
    env_logger::init();

    let token = std::env::var("LZT_API_TOKEN").unwrap_or_else(|_| "your_token".to_string());

    let client = ApiClient::builder()
        .base_url("https://api.lolz.live")
        .token(&token)
        .retry(3)
        .build()?;

    let forum = ForumClient::new(client);

    println!("=== Forum API Demo ===\n");

    println!("Fetching categories...");
    match forum.get_categories().await {
        Ok(response) => {
            println!("Total categories: {}", response.categories_total);
            for category in response.categories.iter().take(5) {
                println!(
                    "  - {} (ID: {})",
                    category.category_title, category.category_id
                );
            }
        }
        Err(e) => eprintln!("Error fetching categories: {}", e),
    }

    println!("\nFetching forums...");
    match forum.get_forums().await {
        Ok(response) => {
            println!("Total forums: {}", response.forums_total);
            for forum in response.forums.iter().take(5) {
                println!("  - {} (ID: {})", forum.forum_title, forum.forum_id);
            }
        }
        Err(e) => eprintln!("Error fetching forums: {}", e),
    }

    println!("\nFetching current user...");
    match forum.get_me().await {
        Ok(response) => println!(
            "Logged in as: {} (ID: {})",
            response.user.username, response.user.user_id
        ),
        Err(e) => eprintln!("Error fetching user: {}", e),
    }

    Ok(())
}
