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

    // Actual Monoova API endpoint paths (verified against TypeScript SDK)
    let endpoints = vec![
        ("M Account List", "/mAccount/v1/listAsIssuer"),
        ("M Account Financials", "/mAccount/v1/financials/6279059733176454"),
        ("M Account Get", "/mAccount/v1/get/6279059733176454"),
        ("Subscriptions List", "/subscriptions/v1/list"),
        ("Receivables Create", "/receivables/v1/create"),
        ("Receivables Status", "/receivables/v1/statusByClientID/test-user-id"),
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
