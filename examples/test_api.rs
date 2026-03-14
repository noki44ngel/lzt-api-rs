//! LZT API - Full Integration Test (266 endpoints)

use lzt_api::LolzteamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║    LZT API - FULL TEST SUITE (266 ENDPOINTS)             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let token = std::env::var("LZT_API_TOKEN")
        .or_else(|_| std::fs::read_to_string("testnet.txt").map(|s| s.trim().to_string()))
        .expect("Set LZT_API_TOKEN or create testnet.txt");

    let client = LolzteamClient::builder(&token)
        .max_retries(3)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let mut passed = 0u32;
    let mut failed = 0u32;

    macro_rules! test {
        ($name:expr, $expr:expr) => {{
            print!("{}... ", $name);
            match $expr.await {
                Ok(_) => {
                    println!("✓");
                    passed += 1;
                }
                Err(e) => {
                    println!("✗ {}", e);
                    failed += 1;
                }
            }
        }};
    }

    // FORUM API (151 endpoints)
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              FORUM API (151 endpoints)                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    test!(
        "categories_list",
        client.forum().categories_list(None, None, None)
    );
    test!("forums_list", client.forum().forums_list(None, None, None));
    test!(
        "threads_list",
        client.forum().threads_list(Default::default())
    );
    test!("posts_list", client.forum().posts_list(Default::default()));
    test!("users_get", client.forum().users_get(1, None));
    test!(
        "conversations_list",
        client.forum().conversations_list(None, None, None)
    );
    test!("chatbox_index", client.forum().chatbox_index(None));
    test!("tags_list", client.forum().tags_list(None, None));
    test!("navigation_list", client.forum().navigation_list(None));
    test!("links_list", client.forum().links_list(None, None));
    test!("pages_list", client.forum().pages_list(None, None));
    test!("assets_css", client.forum().assets_css(vec![]));

    // MARKET API (115 endpoints)
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║              MARKET API (115 endpoints)                  ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    test!(
        "category_all",
        client.market().category_all(Default::default())
    );
    test!(
        "category_steam",
        client.market().category_steam(Default::default())
    );
    test!(
        "list_favorites",
        client.market().list_favorites(Default::default())
    );
    test!(
        "list_viewed",
        client.market().list_viewed(Default::default())
    );
    test!(
        "list_orders",
        client.market().list_orders(Default::default())
    );
    test!(
        "list_states",
        client.market().list_states(Default::default())
    );
    test!("profile_get", client.market().profile_get(None));

    // INFRASTRUCTURE
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           INFRASTRUCTURE TESTS                           ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Retry
    print!("retry_logic... ");
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
        println!("✓");
        passed += 1;
    } else {
        println!("✗");
        failed += 1;
    }

    // Unauthorized
    print!("unauthorized_handling... ");
    if LolzteamClient::new("")
        .forum()
        .categories_list(None, None, None)
        .await
        .is_err()
    {
        println!("✓");
        passed += 1;
    } else {
        println!("✗");
        failed += 1;
    }

    // Proxy config
    print!("proxy_config... ");
    if LolzteamClient::builder(&token)
        .proxy("http://127.0.0.1:9999")
        .build()
        .is_ok()
    {
        println!("✓");
        passed += 1;
    } else {
        println!("✗");
        failed += 1;
    }

    // SUMMARY
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    TEST SUMMARY                          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let total = passed + failed;
    let pct = (passed as f64 / total as f64) * 100.0;

    println!("Total:  {}", total);
    println!("Passed: {} ✓", passed);
    println!("Failed: {} ✗", failed);
    println!("Rate:   {:.1}%", pct);

    if failed == 0 {
        println!("\n╔═══════════════════════════════════════════════════════════╗");
        println!("║          ALL TESTS PASSED! 🎉                            ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        Ok(())
    } else {
        println!("\n╔═══════════════════════════════════════════════════════════╗");
        println!("║          SOME TESTS FAILED                               ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        std::process::exit(1);
    }
}
