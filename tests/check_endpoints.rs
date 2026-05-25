//! Monoova API endpoint verification script
//!
//! Debugging tool to verify actual API endpoint paths

use monoova_rs_sdk::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("MONOOVA_API_KEY")
        .expect("MONOOVA_API_KEY environment variable is required");

    let base_url = env::var("MONOOVA_BASE_URL")
        .unwrap_or_else(|_| "https://api.m-pay.com.au".to_string());

    let client = Client::with_base_url(api_key, base_url)?;

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Monoova API Endpoint Check");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Base URL: {}", client.base_url());
    println!();

    // Try common endpoint paths
    let endpoints = vec![
        ("Financial Balance", "/financial/v1/balance"),
        ("Financial Balance (alt)", "/api/financial/v1/balance"),
        ("Financial Balance (no version)", "/financial/balance"),
        ("Account Info", "/account/v1"),
        ("Account Info (alt)", "/api/account/v1"),
        ("M Account List", "/maccount/v1"),
        ("M Account List (alt)", "/api/maccount/v1"),
    ];

    for (name, path) in endpoints {
        let full_url = format!("{}{}", client.base_url(), path);
        println!("Testing: {} -> {}", name, full_url);

        match client.get::<serde_json::Value>(path).await {
            Ok(_) => {
                println!("  ✅ SUCCESS - endpoint path looks correct!");
            }
            Err(e) => {
                if let monoova_rs_sdk::ApiError::Api { status, .. } = &e {
                    println!("  ❌ Status: {} - {}", status, e);
                } else {
                    println!("  ❌ Error: {}", e);
                }
            }
        }
        println!();
    }

    Ok(())
}
