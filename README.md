# Monoova Rust SDK

Rust SDK for the [Monoova Payments API](https://developer.monoova.com/). Provides a type-safe, async interface for interacting with Monoova's payment services.

> **Reference implementation**: This SDK is modelled after the JavaScript SDK — [`sdksio-monoova-sdk`](https://www.npmjs.com/package/sdksio-monoova-sdk) on npm.

---

## Features

- ✅ Full async/await support with Tokio
- ✅ Type-safe API client
- ✅ Comprehensive error handling
- ✅ Retry logic with exponential backoff
- ✅ Production and Sandbox environment support
- ✅ Basic Authentication

---

## Installation

```toml
[dependencies]
monoova-rs-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

---

## Quick Start

```rust
use monoova_rs_sdk::{Client, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_api_key_and_env(
        "your-api-key".to_string(),
        Environment::Sandbox,
    )?;

    let balance = client.financial().get_balance().await?;
    println!("Balance: {:.2} {}", balance.balance, balance.currency);

    Ok(())
}
```

---

## API Coverage

Based on the JS SDK reference. The following table shows implementation status per module.

> **Note**: Incoming webhook event handling and `transactionExecute` are intentionally out of scope — these involve sensitive fund movement and server-side event processing that are better handled at the application layer for security reasons.

| Module                        | JS SDK methods | Rust SDK methods | Status                                                        |
| ----------------------------- | -------------- | ---------------- | ------------------------------------------------------------- |
| `subscriptions`               | 6              | 6                | ✅ Full                                                       |
| `security`                    | 2              | 2                | ✅ Full                                                       |
| `reports`                     | 2              | 3                | ✅ Full (+`get_status`, `download`)                           |
| `financial`                   | 5              | 4                | 🟡 Partial — read-only (no `transactionExecute`, intentional) |
| `bpay`                        | 6              | 3                | 🟡 Partial                                                    |
| `payid`                       | 4              | 3                | 🟡 Partial                                                    |
| `token`                       | 8              | 3                | 🟡 Partial                                                    |
| `verify`                      | 6              | 2                | 🟡 Partial                                                    |
| `m_account`                   | 8              | 4                | 🟡 Partial — read-only                                        |
| `m_wallet`                    | 8              | 4                | 🟡 Partial                                                    |
| `webhooks`                    | 11\*           | 5                | 🟡 Subscription management only\*                             |
| `automatcher` + `receivables` | 19             | 10               | 🟡 Partial                                                    |
| `tools`                       | 4              | 2                | 🟡 Partial                                                    |
| `reconciliation_rules`        | 4              | 1                | 🔴 Minimal                                                    |
| `testing` / ping              | 3              | 0                | 🔴 Not implemented                                            |

> \* The JS SDK's `webhooks` module handles **incoming event payloads** (e.g. `receivablesReceivePaymentWebhook`, `nppPaymentStatus`). The Rust SDK's `webhooks` module handles **subscription CRUD** (create/get/list/update/delete). These serve different purposes; incoming event processing is left to the application layer.

---

## API Modules

### Financial API

```rust
// Get account balance
let balance = client.financial().get_balance().await?;

// Get account statement
let statement = client.financial().get_statement(&StatementParams {
    from_date: Some("2024-01-01".to_string()),
    to_date: Some("2024-12-31".to_string()),
    page: Some(1),
    page_size: Some(50),
}).await?;

// Get transaction details
let tx = client.financial().get_transaction("transaction-id").await?;

// Get transactions list
let txs = client.financial().get_transactions(&TransactionParams {
    from_date: Some("2024-01-01".to_string()),
    to_date: Some("2024-12-31".to_string()),
    transaction_type: None,
    page: Some(1),
    page_size: Some(50),
}).await?;
```

### BPAY API

```rust
// Create a BPAY payment
let payment = client.bpay().create_payment(&BpayPaymentRequest {
    amount: 100.50,
    biller_code: "12345".to_string(),
    customer_reference: "REF123".to_string(),
    description: Some("Invoice payment".to_string()),
}).await?;

// Get payment status
let status = client.bpay().get_payment_status(&payment.payment_id).await?;

// Get biller details
let biller = client.bpay().get_biller_details("12345").await?;
```

### Pay ID API

```rust
// Resolve a Pay ID
let resolution = client.payid().resolve("user@example.com").await?;

// Create a Pay ID payment
let payment = client.payid().create_payment(&PayIdPaymentRequest {
    pay_id: "user@example.com".to_string(),
    amount: 50.00,
    description: Some("Payment".to_string()),
    reference: Some("REF123".to_string()),
}).await?;

// Get payment status
let status = client.payid().get_payment_status(&payment.payment_id).await?;
```

### Verify API

```rust
// Verify a bank account
let result = client.verify().verify_account(&VerifyAccountRequest {
    account_number: "12345678".to_string(),
    bsb: "123456".to_string(),
    account_name: Some("John Doe".to_string()),
}).await?;

// Verify a BSB
let bsb = client.verify().verify_bsb("123456").await?;
```

### Webhooks API (Subscription Management)

```rust
// Register a webhook endpoint
let webhook = client.webhooks().create(&CreateWebhookRequest {
    url: "https://example.com/webhook".to_string(),
    events: vec!["payment.received".to_string()],
    active: Some(true),
}).await?;

// List registered webhooks
let list = client.webhooks().list(&WebhookListParams {
    page: Some(1),
    page_size: Some(50),
    active: Some(true),
}).await?;

// Update a webhook
client.webhooks().update(&webhook.webhook_id, &UpdateWebhookRequest {
    active: Some(false),
    ..Default::default()
}).await?;

// Delete a webhook
client.webhooks().delete(&webhook.webhook_id).await?;
```

### M Account API

```rust
// List M Accounts
let accounts = client.m_account().list().await?;

// Get M Account details
let account = client.m_account().get("account-number").await?;

// Get M Account financials
let financials = client.m_account().get_financials("account-number").await?;

// Get M Account transactions
let txs = client.m_account().get_transactions(&MAccountTransactionsRequest {
    account_number: "account-number".to_string(),
    frequency: "Daily".to_string(),
    descending: Some(true),
    use_time: None,
    start_date: Some("2024-01-01".to_string()),
    end_date: Some("2024-12-31".to_string()),
}).await?;
```

### M Wallet API

```rust
// Create an M Wallet
let wallet = client.m_wallet().create(&CreateMWalletRequest {
    wallet_name: "My Wallet".to_string(),
    description: Some("Wallet description".to_string()),
}).await?;

// Get wallet details
let wallet = client.m_wallet().get("wallet-id").await?;

// List wallets
let wallets = client.m_wallet().list().await?;

// Transfer funds
let transfer = client.m_wallet().transfer(&WalletTransferRequest {
    from_wallet_id: "wallet-id-1".to_string(),
    to_wallet_id: "wallet-id-2".to_string(),
    amount: 100.00,
    description: Some("Transfer".to_string()),
}).await?;
```

### Subscriptions API

```rust
// Create a subscription
let sub = client.subscriptions().create(&CreateSubscriptionRequest {
    name: "Monthly Payment".to_string(),
    amount: 50.00,
    frequency: "monthly".to_string(),
    start_date: "2024-01-01".to_string(),
    end_date: None,
    description: Some("Subscription".to_string()),
}).await?;

// List subscriptions
let subs = client.subscriptions().list().await?;

// Delete a subscription
client.subscriptions().delete(&sub.subscription_id).await?;

// Resend a subscription notification
client.subscriptions().resend(&sub.subscription_id).await?;

// Get subscription report
let report = client.subscriptions().get_report().await?;
```

### Security API

```rust
// Generate a one-shot security token
let token = client.security().generate_oneshot_token().await?;

// Change password
client.security().change_password(&ChangePasswordRequest {
    current_password: "old".to_string(),
    new_password: "new".to_string(),
}).await?;
```

### Token API

```rust
// Create a token
let token = client.token().create(&CreateTokenRequest {
    name: "my-token".to_string(),
    expires_in: Some(3600),
    permissions: Some(vec!["read".to_string()]),
}).await?;

// Get token details
let token = client.token().get("token-id").await?;

// Revoke a token
client.token().revoke("token-id").await?;
```

### Tools API

```rust
// Validate a BSB number
let result = client.tools().validate_bsb("062000").await?;

// Validate a bank account number
let result = client.tools().validate_account("12345678", "062000").await?;
```

### Reports API

```rust
// Generate a report
let report = client.reports().generate(&GenerateReportRequest {
    report_type: "settlement".to_string(),
    from_date: "2024-01-01".to_string(),
    to_date: "2024-12-31".to_string(),
}).await?;

// Check report generation status
let status = client.reports().get_status(&report.report_id).await?;

// Download the report
let data = client.reports().download(&report.report_id).await?;
```

### Automatcher + Receivables API

```rust
// Get bank account receivables
let receivables = client.automatcher().get_receivables(&ReceivablesParams {
    from_date: Some("2024-01-01".to_string()),
    to_date: Some("2024-12-31".to_string()),
    page: Some(1),
    page_size: Some(50),
}).await?;

// Add a whitelist entry
let entry = client.automatcher().create_whitelist(&CreateWhitelistRequest {
    account_number: "12345678".to_string(),
    bsb: "062000".to_string(),
    account_name: Some("ACME Corp".to_string()),
}).await?;

// Create a receivables account
let account = client.receivables().create_account(&CreateReceivablesAccountRequest {
    ..Default::default()
}).await?;

// Refund a receivable
client.receivables().refund(&RefundRequest {
    ..Default::default()
}).await?;
```

---

## Environments

| Environment               | Base URL                   |
| ------------------------- | -------------------------- |
| `Environment::Production` | `https://api.mpay.com.au`  |
| `Environment::Sandbox`    | `https://api.m-pay.com.au` |

---

## Authentication

```rust
// API key only (most common)
let client = Client::with_api_key("your-api-key".to_string())?;

// API key + environment
let client = Client::with_api_key_and_env(
    "your-api-key".to_string(),
    Environment::Sandbox,
)?;

// Full credentials
let credentials = BasicAuthCredentials::new("username".to_string(), "password".to_string());
let config = ClientConfig::new(credentials, None, None, None);
let client = Client::new(config)?;
```

---

## Error Handling

```rust
match client.financial().get_balance().await {
    Ok(balance) => println!("Balance: {}", balance.balance),
    Err(e) => match e {
        ApiError::Http(e) => eprintln!("HTTP error: {}", e),
        ApiError::Api { status, message } => eprintln!("API error {}: {}", status, message),
        ApiError::Authentication(msg) => eprintln!("Auth error: {}", msg),
        _ => eprintln!("Error: {}", e),
    },
}
```

---

## Retry Logic

```rust
let retry_config = RetryConfiguration {
    max_number_of_retries: 3,
    retry_on_timeout: true,
    retry_interval: 1,
    maximum_retry_wait_time: 10,
    backoff_factor: 2,
    http_status_codes_to_retry: vec![408, 429, 500, 502, 503, 504],
    http_methods_to_retry: vec!["GET".to_string(), "PUT".to_string()],
};

let http_options = HttpClientOptions {
    timeout: Some(30),
    retry_config: Some(retry_config),
};

let config = ClientConfig::new(
    BasicAuthCredentials::from_api_key("your-api-key".to_string()),
    Some(Environment::Sandbox),
    Some(30),
    Some(http_options),
);

let client = Client::new(config)?;
```

---

## Live API Testing

A convenience script `test_live_tem.sh` is included in the repo for running integration tests against the real Monoova API.

### 1. Edit the script with your credentials

```bash
# test_live_tem.sh
#!/bin/bash

export MONOOVA_BASE_URL="YOUR_MONOOVA_BASE_URL"
export MONOOVA_API_KEY="YOUR_MONOOVA_API_KEY"
export MONOOVA_ACCOUNT_NUMBER="YOUR_MONOOVA_ACCOUNT_NUMBER"
export MONOOVA_BSB="YOUR_MONOOVA_BSB"

echo "🚀 Monoova Live API Testing..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo test --test live_api_test -- --nocapture --ignored

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Test Complete!"
```

### 2. Run it

```bash
chmod +x test_live_tem.sh
./test_live_tem.sh
```

### Run specific tests manually

```bash
# All live tests
MONOOVA_API_KEY="your-api-key" cargo test --test live_api_test -- --ignored --nocapture

# Single test
MONOOVA_API_KEY="your-api-key" cargo test --test live_api_test test_live_financial_get_balance -- --ignored --nocapture
```

> Live API tests are gated with `#[ignore]` by default — pass `--ignored` to run them.

---

## References

- [Monoova API Documentation](https://developer.monoova.com/)
- [Reference JS SDK — `sdksio-monoova-sdk` on npm](https://www.npmjs.com/package/sdksio-monoova-sdk)

---

## License

MIT
