use monoova_rs_sdk::{
    apis::{
        automatcher::ReceivablesParams,
        m_account::CreateMAccountRequest,
        m_wallet::{CreateMWalletRequest, WalletTransferRequest},
        subscriptions::CreateSubscriptionRequest,
        webhooks::CreateWebhookRequest,
    },
    Client,
};
use mockito::{Matcher, Server};
use serde_json::json;

#[tokio::test]
async fn test_m_account_create() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/maccount/v1")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "account_id": "acc-123",
                "account_name": "Test Account",
                "account_number": "12345678",
                "bsb": "123456",
                "balance": 0.0,
                "created_at": "2024-01-01T00:00:00Z"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = CreateMAccountRequest {
        account_name: "Test Account".to_string(),
        account_number: None,
        bsb: None,
        description: Some("Test description".to_string()),
    };
    let result = client.m_account().create(&request).await;

    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.account_id, "acc-123");
    assert_eq!(account.account_name, "Test Account");

    mock.assert();
}

#[tokio::test]
async fn test_m_wallet_create() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/mwallet/v1")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "wallet_id": "wallet-123",
                "wallet_name": "Test Wallet",
                "balance": 0.0,
                "currency": "AUD",
                "created_at": "2024-01-01T00:00:00Z"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = CreateMWalletRequest {
        wallet_name: "Test Wallet".to_string(),
        description: Some("Test description".to_string()),
    };
    let result = client.m_wallet().create(&request).await;

    assert!(result.is_ok());
    let wallet = result.unwrap();
    assert_eq!(wallet.wallet_id, "wallet-123");
    assert_eq!(wallet.wallet_name, "Test Wallet");

    mock.assert();
}

#[tokio::test]
async fn test_m_wallet_transfer() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/mwallet/v1/transfer")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "transfer_id": "transfer-123",
                "from_wallet_id": "wallet-1",
                "to_wallet_id": "wallet-2",
                "amount": 100.0,
                "status": "completed",
                "created_at": "2024-01-01T00:00:00Z"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = WalletTransferRequest {
        from_wallet_id: "wallet-1".to_string(),
        to_wallet_id: "wallet-2".to_string(),
        amount: 100.0,
        description: Some("Test transfer".to_string()),
    };
    let result = client.m_wallet().transfer(&request).await;

    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert_eq!(transfer.transfer_id, "transfer-123");
    assert_eq!(transfer.amount, 100.0);

    mock.assert();
}

#[tokio::test]
async fn test_subscriptions_create() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/subscriptions/v1")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "subscription_id": "sub-123",
                "name": "Monthly Payment",
                "amount": 50.0,
                "frequency": "monthly",
                "status": "active",
                "created_at": "2024-01-01T00:00:00Z"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = CreateSubscriptionRequest {
        name: "Monthly Payment".to_string(),
        amount: 50.0,
        frequency: "monthly".to_string(),
        start_date: "2024-01-01".to_string(),
        end_date: None,
        description: Some("Test subscription".to_string()),
    };
    let result = client.subscriptions().create(&request).await;

    assert!(result.is_ok());
    let subscription = result.unwrap();
    assert_eq!(subscription.subscription_id, "sub-123");
    assert_eq!(subscription.amount, 50.0);

    mock.assert();
}

#[tokio::test]
async fn test_webhooks_create() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/webhooks/v1")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "webhook_id": "webhook-123",
                "url": "https://example.com/webhook",
                "events": ["payment.received", "payment.completed"],
                "active": true,
                "created_at": "2024-01-01T00:00:00Z"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = CreateWebhookRequest {
        url: "https://example.com/webhook".to_string(),
        events: vec!["payment.received".to_string(), "payment.completed".to_string()],
        active: Some(true),
    };
    let result = client.webhooks().create(&request).await;

    assert!(result.is_ok());
    let webhook = result.unwrap();
    assert_eq!(webhook.webhook_id, "webhook-123");
    assert_eq!(webhook.events.len(), 2);

    mock.assert();
}

#[tokio::test]
async fn test_automatcher_get_receivables() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/automatcher/v1/receivables")
        .match_query(Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "receivables": [
                    {
                        "id": "rec-123",
                        "amount": 100.0,
                        "account_number": "12345678",
                        "bsb": "123456",
                        "received_at": "2024-01-01T00:00:00Z",
                        "status": "pending"
                    }
                ],
                "total_count": 1,
                "page": 1,
                "page_size": 50
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let params = ReceivablesParams {
        from_date: Some("2024-01-01".to_string()),
        to_date: Some("2024-12-31".to_string()),
        page: Some(1),
        page_size: Some(50),
    };
    let result = client.automatcher().get_receivables(&params).await;

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
    let receivables = result.unwrap();
    assert_eq!(receivables.receivables.len(), 1);
    assert_eq!(receivables.receivables[0].id, "rec-123");

    mock.assert();
}

/// Helper function to create a test client with mock server
fn create_test_client(server: &Server) -> Client {
    Client::with_base_url("test-api-key".to_string(), server.url()).unwrap()
}

