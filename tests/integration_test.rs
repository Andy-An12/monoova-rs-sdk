use monoova_rs_sdk::{
    apis::financial::StatementParams,
    Client, Environment,
};
use mockito::{Matcher, Server};
use serde_json::json;

#[tokio::test]
async fn test_financial_get_balance() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/financial/v1/balance")
        .match_header("authorization", Matcher::Regex(r"Basic.*".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "balance": 1000.50,
                "available_balance": 950.00,
                "currency": "AUD"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let result = client.financial().get_balance().await;

    assert!(result.is_ok());
    let balance = result.unwrap();
    assert_eq!(balance.balance, 1000.50);
    assert_eq!(balance.available_balance, 950.00);
    assert_eq!(balance.currency, "AUD");

    mock.assert();
}

#[tokio::test]
async fn test_financial_get_statement() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/financial/v1/statement")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("from_date".to_string(), "2024-01-01".to_string()),
            Matcher::UrlEncoded("to_date".to_string(), "2024-12-31".to_string()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "transactions": [
                    {
                        "id": "txn-123",
                        "amount": 100.0,
                        "currency": "AUD",
                        "transaction_type": "credit",
                        "description": "Test transaction",
                        "created_at": "2024-01-01T00:00:00Z",
                        "status": "completed"
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
    let params = StatementParams {
        from_date: Some("2024-01-01".to_string()),
        to_date: Some("2024-12-31".to_string()),
        page: Some(1),
        page_size: Some(50),
    };
    let result = client.financial().get_statement(&params).await;

    assert!(result.is_ok());
    let statement = result.unwrap();
    assert_eq!(statement.transactions.len(), 1);
    assert_eq!(statement.transactions[0].id, "txn-123");

    mock.assert();
}

#[tokio::test]
async fn test_bpay_create_payment() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/bpay/v1/payment")
        .match_header("content-type", "application/json")
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "payment_id": "bpay-123",
                "status": "pending",
                "amount": 100.50,
                "created_at": "2024-01-01T00:00:00Z"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = monoova_rs_sdk::apis::bpay::BpayPaymentRequest {
        amount: 100.50,
        biller_code: "12345".to_string(),
        customer_reference: "REF123".to_string(),
        description: Some("Test payment".to_string()),
    };
    let result = client.bpay().create_payment(&request).await;

    assert!(result.is_ok());
    let payment = result.unwrap();
    assert_eq!(payment.payment_id, "bpay-123");
    assert_eq!(payment.amount, 100.50);

    mock.assert();
}

#[tokio::test]
async fn test_verify_account() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/verify/v1/account")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "valid": true,
                "account_name": "John Doe",
                "message": "Account verified"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let request = monoova_rs_sdk::apis::verify::VerifyAccountRequest {
        account_number: "12345678".to_string(),
        bsb: "123456".to_string(),
        account_name: Some("John Doe".to_string()),
    };
    let result = client.verify().verify_account(&request).await;

    assert!(result.is_ok());
    let verification = result.unwrap();
    assert!(verification.valid);
    assert_eq!(verification.account_name, Some("John Doe".to_string()));

    mock.assert();
}

#[tokio::test]
async fn test_payid_resolve() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/payid/v1/resolve/test@example.com")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "pay_id": "test@example.com",
                "account_number": "12345678",
                "bsb": "123456",
                "account_name": "Test Account"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let result = client.payid().resolve("test@example.com").await;

    assert!(result.is_ok());
    let resolution = result.unwrap();
    assert_eq!(resolution.pay_id, "test@example.com");
    assert_eq!(resolution.account_number, "12345678");

    mock.assert();
}

#[tokio::test]
async fn test_api_error_handling() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/financial/v1/balance")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "error": "Unauthorized",
                "message": "Invalid API key"
            })
            .to_string(),
        )
        .create();

    let client = create_test_client(&server);
    let result = client.financial().get_balance().await;

    assert!(result.is_err());
    if let Err(monoova_rs_sdk::ApiError::Api { status, .. }) = result {
        assert_eq!(status, 401);
    } else {
        panic!("Expected ApiError::Api");
    }

    mock.assert();
}

#[tokio::test]
async fn test_client_with_different_environments() {
    let client_prod = monoova_rs_sdk::Client::with_api_key_and_env(
        "test-key".to_string(),
        Environment::Production,
    )
    .unwrap();
    assert_eq!(client_prod.base_url(), "https://api.mpay.com.au");

    let client_sandbox = monoova_rs_sdk::Client::with_api_key_and_env(
        "test-key".to_string(),
        Environment::Sandbox,
    )
    .unwrap();
    assert_eq!(client_sandbox.base_url(), "https://api.m-pay.com.au");
}

/// Helper function to create a test client with mock server
fn create_test_client(server: &Server) -> Client {
    Client::with_base_url("test-api-key".to_string(), server.url()).unwrap()
}

