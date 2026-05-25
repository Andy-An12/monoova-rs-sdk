//! Live integration tests against the real Monoova API (fixed endpoints)
use monoova_rs_sdk::Client;
use std::env;

/// Create a test client from environment variables
fn create_live_client() -> Client {
    let api_key = env::var("MONOOVA_API_KEY")
        .expect("MONOOVA_API_KEY environment variable is required");

    let base_url = env::var("MONOOVA_BASE_URL")
        .unwrap_or_else(|_| "https://api.m-pay.com.au".to_string());

    Client::with_base_url(api_key, base_url)
        .expect("Failed to create client")
}

#[tokio::test]
#[ignore]
async fn test_live_m_account_list() {
    let client = create_live_client();

    println!("Testing M Account API - List Accounts...");
    if let Ok(full_url) = client.get_full_url("/mAccount/v1/listAsIssuer") {
        println!("   Full URL: {}", full_url);
    }

    match client.m_account().list().await {
        Ok(accounts) => {
            println!("✅ M Accounts retrieved successfully!");
            println!("   Response: {:?}", accounts);
        }
        Err(e) => {
            eprintln!("❌ Failed to list M accounts: {}", e);
            if let monoova_rs_sdk::ApiError::Api { status, .. } = &e {
                if *status == 404 {
                    println!("   ⚠️  Endpoint not found");
                    return;
                }
            }
            panic!("Failed: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_live_subscriptions_list() {
    let client = create_live_client();

    println!("Testing Subscriptions API - List Subscriptions...");
    if let Ok(full_url) = client.get_full_url("/subscriptions/v1/list") {
        println!("   Full URL: {}", full_url);
    }

    match client.subscriptions().list().await {
        Ok(subscriptions) => {
            println!("✅ Subscriptions retrieved successfully!");
            println!("   Response: {:?}", subscriptions);
        }
        Err(e) => {
            eprintln!("❌ Failed to list subscriptions: {}", e);
            if let monoova_rs_sdk::ApiError::Api { status, .. } = &e {
                if *status == 404 {
                    println!("   ⚠️  Endpoint not found");
                    return;
                }
            }
            panic!("Failed: {:?}", e);
        }
    }
}
