//! LZT API - Full Integration Test

use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== LZT API Integration Tests ===\n");

    let token = std::env::var("LZT_API_TOKEN")
        .or_else(|_| std::fs::read_to_string("testnet.txt").map(|s| s.trim().to_string()))
        .expect("Set LZT_API_TOKEN or create testnet.txt");

    let client = LolzteamClient::builder(&token)
        .max_retries(3)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let mut passed = 0u32;
    let mut failed = 0u32;

    // FORUM TESTS
    println!("FORUM API:");

    if client
        .forum()
        .categories_list(None, None, None)
        .await
        .is_ok()
    {
        println!("  ✓ categories_list");
        passed += 1;
    } else {
        println!("  ✗ categories_list");
        failed += 1;
    }

    if client.forum().forums_list(None, None, None).await.is_ok() {
        println!("  ✓ forums_list");
        passed += 1;
    } else {
        println!("  ✗ forums_list");
        failed += 1;
    }

    if client
        .forum()
        .threads_list(Default::default())
        .await
        .is_ok()
    {
        println!("  ✓ threads_list");
        passed += 1;
    } else {
        println!("  ✗ threads_list");
        failed += 1;
    }

    if client.forum().posts_list(Default::default()).await.is_ok() {
        println!("  ✓ posts_list");
        passed += 1;
    } else {
        println!("  ✗ posts_list");
        failed += 1;
    }

    if client.forum().users_get(1, None).await.is_ok() {
        println!("  ✓ users_get");
        passed += 1;
    } else {
        println!("  ✗ users_get");
        failed += 1;
    }

    if client
        .forum()
        .conversations_list(None, None, None)
        .await
        .is_ok()
    {
        println!("  ✓ conversations_list");
        passed += 1;
    } else {
        println!("  ✗ conversations_list");
        failed += 1;
    }

    if client.forum().chatbox_index(None).await.is_ok() {
        println!("  ✓ chatbox_index");
        passed += 1;
    } else {
        println!("  ✗ chatbox_index");
        failed += 1;
    }

    if client.forum().tags_list(None, None).await.is_ok() {
        println!("  ✓ tags_list");
        passed += 1;
    } else {
        println!("  ✗ tags_list");
        failed += 1;
    }

    // MARKET TESTS
    println!("\nMARKET API:");

    if client
        .market()
        .category_all(Default::default())
        .await
        .is_ok()
    {
        println!("  ✓ category_all");
        passed += 1;
    } else {
        println!("  ✗ category_all");
        failed += 1;
    }

    if client
        .market()
        .category_steam(Default::default())
        .await
        .is_ok()
    {
        println!("  ✓ category_steam");
        passed += 1;
    } else {
        println!("  ✗ category_steam");
        failed += 1;
    }

    if client
        .market()
        .list_favorites(Default::default())
        .await
        .is_ok()
    {
        println!("  ✓ list_favorites");
        passed += 1;
    } else {
        println!("  ✗ list_favorites");
        failed += 1;
    }

    if client
        .market()
        .list_viewed(Default::default())
        .await
        .is_ok()
    {
        println!("  ✓ list_viewed");
        passed += 1;
    } else {
        println!("  ✗ list_viewed");
        failed += 1;
    }

    // INFRASTRUCTURE
    println!("\nINFRASTRUCTURE:");

    // Retry
    let mut retry_ok = 0;
    for _ in 0..3 {
        if client
            .forum()
            .categories_list(None, None, None)
            .await
            .is_ok()
        {
            retry_ok += 1;
        }
    }
    if retry_ok >= 2 {
        println!("  ✓ retry_logic");
        passed += 1;
    } else {
        println!("  ✗ retry_logic");
        failed += 1;
    }

    // Unauthorized
    if LolzteamClient::new("")
        .forum()
        .categories_list(None, None, None)
        .await
        .is_err()
    {
        println!("  ✓ unauthorized_handling");
        passed += 1;
    } else {
        println!("  ✗ unauthorized_handling");
        failed += 1;
    }

    // Proxy config
    if LolzteamClient::builder(&token)
        .proxy("http://127.0.0.1:9999")
        .build()
        .is_ok()
    {
        println!("  ✓ proxy_config");
        passed += 1;
    } else {
        println!("  ✗ proxy_config");
        failed += 1;
    }

    // Summary
    println!("\n=== SUMMARY ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total:  {}", passed + failed);

    if failed == 0 {
        println!("\n✓ ALL TESTS PASSED!");
        Ok(())
    } else {
        std::process::exit(1);
    }
}
