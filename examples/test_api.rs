use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("LZT_API_TOKEN").expect("LZT_API_TOKEN must be set");
    
    println!("=== LZT API Integration Tests ===\n");
    
    let client = LolzteamClient::builder(&token)
        .max_retries(3)
        .build()?;

    println!("Test 1: Forum categories...");
    match client.forum().categories_list(None, None, None).await {
        Ok(_) => println!("  ✓ Success"),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("Test 2: Forums...");
    match client.forum().forums_list(None, None, None).await {
        Ok(_) => println!("  ✓ Success"),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\nTest 3: Market items...");
    match client.market().category_all(Default::default()).await {
        Ok(_) => println!("  ✓ Success"),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\nTest 4: Steam items...");
    match client.market().category_steam(Default::default()).await {
        Ok(_) => println!("  ✓ Success"),
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\n=== All Tests Completed ===");
    Ok(())
}
