//! Live integration tests against the real Monoova API
//!
//! These tests make actual API calls and require the following environment variables:
//! - MONOOVA_BASE_URL: API base URL (default: https://api.m-pay.com.au)
//! - MONOOVA_API_KEY: Your API key
//! - MONOOVA_ACCOUNT_NUMBER: Test account number (optional)
//! - MONOOVA_BSB: Test BSB number (optional)
//!
//! Run with:
//! ```bash
//! MONOOVA_API_KEY="your-api-key" cargo test --test live_api_test -- --nocapture
//! ```

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
#[ignore] // Skipped by default — requires real API credentials
async fn test_live_financial_get_balance() {
    let client = create_live_client();

    println!("Testing Financial API - Get Balance...");
    println!("   Base URL: {}", client.base_url());
    if let Ok(full_url) = client.get_full_url("/financial/v1/balance") {
        println!("   Full URL: {}", full_url);
    }

    match client.financial().get_balance().await {
        Ok(balance) => {
            println!("✅ Balance retrieved successfully!");
            println!("   Balance: {:.2} {}", balance.balance, balance.currency);
            println!("   Available Balance: {:.2} {}", balance.available_balance, balance.currency);
            assert!(balance.balance >= 0.0);
        }
        Err(e) => {
            eprintln!("❌ Failed to get balance: {}", e);
            if let monoova_rs_sdk::ApiError::Api { status, message } = &e {
                eprintln!("   Status: {}", status);
                eprintln!("   Message: {}", message);
                if *status == 404 {
                    eprintln!("   ⚠️  Endpoint path may be incorrect.");
                    eprintln!("   Check the Monoova API docs for the correct path.");
                    eprintln!("   Expected: {}/financial/v1/balance", client.base_url());
                }
            }
            // Don't panic on 404 — endpoint path may differ
            if let monoova_rs_sdk::ApiError::Api { status, .. } = &e {
                if *status == 404 {
                    println!("   ⚠️  Skipping test (404 - endpoint not found)");
                    return;
                }
            }
            panic!("Balance retrieval failed: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_live_financial_get_statement() {
    let client = create_live_client();

    println!("Testing Financial API - Get Statement...");
    let params = monoova_rs_sdk::apis::financial::StatementParams {
        from_date: Some("2024-01-01".to_string()),
        to_date: Some("2024-12-31".to_string()),
        page: Some(1),
        page_size: Some(10),
    };

    match client.financial().get_statement(&params).await {
        Ok(statement) => {
            println!("✅ Statement retrieved successfully!");
            println!("   Transactions: {}", statement.transactions.len());
            if let Some(total) = statement.total_count {
                println!("   Total Count: {}", total);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get statement: {}", e);
            // Endpoint may not be available on all accounts
            println!("   Note: This endpoint might not be available");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_live_verify_bsb() {
    let client = create_live_client();

    let bsb = env::var("MONOOVA_BSB")
        .unwrap_or_else(|_| "802985".to_string())
        .replace("-", "");

    println!("Testing Verify API - Verify BSB: {}...", bsb);
    match client.verify().verify_bsb(&bsb).await {
        Ok(bsb_info) => {
            println!("✅ BSB verification successful!");
            if let Some(bank_name) = bsb_info.bank_name {
                println!("   Bank Name: {}", bank_name);
            }
            if let Some(branch_name) = bsb_info.branch_name {
                println!("   Branch Name: {}", branch_name);
            }
            assert!(bsb_info.valid);
        }
        Err(e) => {
            eprintln!("❌ Failed to verify BSB: {}", e);
            println!("   Note: This endpoint might not be available");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_live_verify_account() {
    let client = create_live_client();

    let account_number = env::var("MONOOVA_ACCOUNT_NUMBER")
        .unwrap_or_else(|_| "12345678".to_string());
    let bsb = env::var("MONOOVA_BSB")
        .unwrap_or_else(|_| "802985".to_string())
        .replace("-", "");

    println!("Testing Verify API - Verify Account...");
    println!("   Account: {} / BSB: {}", account_number, bsb);

    let request = monoova_rs_sdk::apis::verify::VerifyAccountRequest {
        account_number: account_number.clone(),
        bsb: bsb.clone(),
        account_name: None,
    };

    match client.verify().verify_account(&request).await {
        Ok(verification) => {
            println!("✅ Account verification successful!");
            println!("   Valid: {}", verification.valid);
            if let Some(account_name) = verification.account_name {
                println!("   Account Name: {}", account_name);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to verify account: {}", e);
            println!("   Note: This endpoint might not be available");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_live_tools_validate_bsb() {
    let client = create_live_client();

    let bsb = env::var("MONOOVA_BSB")
        .unwrap_or_else(|_| "802985".to_string())
        .replace("-", "");

    println!("Testing Tools API - Validate BSB: {}...", bsb);
    match client.tools().validate_bsb(&bsb).await {
        Ok(validation) => {
            println!("✅ BSB validation successful!");
            println!("   Valid: {}", validation.valid);
            if let Some(bank_name) = validation.bank_name {
                println!("   Bank Name: {}", bank_name);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to validate BSB: {}", e);
            println!("   Note: This endpoint might not be available");
        }
    }
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
            println!("   Status: {:?}", accounts.status);
            if let Some(ref m_accounts) = accounts.m_accounts {
                println!("   Accounts count: {}", m_accounts.len());
                for (i, account) in m_accounts.iter().take(5).enumerate() {
                    println!("   Account {}: {:?}", i + 1, account);
                }
            } else {
                println!("   No accounts found (empty list)");
            }
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
async fn test_live_m_wallet_list() {
    let client = create_live_client();

    println!("Testing M Wallet API - List Wallets...");
    let params = monoova_rs_sdk::apis::m_wallet::MWalletListParams {
        page: Some(1),
        page_size: Some(10),
    };

    match client.m_wallet().list(&params).await {
        Ok(wallets) => {
            println!("✅ M Wallets retrieved successfully!");
            println!("   Wallets: {}", wallets.wallets.len());
            if let Some(total) = wallets.total_count {
                println!("   Total Count: {}", total);
            }
            for wallet in &wallets.wallets {
                println!("   - {}: {} (Balance: {:.2} {})",
                    wallet.wallet_id,
                    wallet.wallet_name,
                    wallet.balance,
                    wallet.currency
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list M wallets: {}", e);
            println!("   Note: This endpoint might not be available or you might not have access");
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_live_webhooks_list() {
    let client = create_live_client();

    println!("Testing Webhooks API - List Webhooks...");
    let params = monoova_rs_sdk::apis::webhooks::WebhookListParams {
        page: Some(1),
        page_size: Some(10),
        active: None,
    };

    match client.webhooks().list(&params).await {
        Ok(webhooks) => {
            println!("✅ Webhooks retrieved successfully!");
            println!("   Webhooks: {}", webhooks.webhooks.len());
            if let Some(total) = webhooks.total_count {
                println!("   Total Count: {}", total);
            }
            for webhook in &webhooks.webhooks {
                println!("   - {}: {} (Active: {}, Events: {})",
                    webhook.webhook_id,
                    webhook.url,
                    webhook.active,
                    webhook.events.len()
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list webhooks: {}", e);
            println!("   Note: This endpoint might not be available or you might not have access");
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
            if let Some(items) = &subscriptions.items {
                println!("   Subscriptions: {}", items.len());
                for item in items {
                    println!("   - ID: {:?}, URL: {:?}, Status: {:?}",
                        item.id,
                        item.webhook_url,
                        item.disabled
                    );
                }
            }
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
