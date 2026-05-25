# Test Guide

This directory contains integration tests for the Monoova Rust SDK.

## Test Structure

- `integration_test.rs`: Integration tests for major API endpoints
- `api_tests.rs`: Tests for various API modules
- `utils_test.rs`: Utility function tests

## Running Tests

### Run all tests
```bash
cargo test
```

### Run a specific test file
```bash
cargo test --test integration_test
cargo test --test api_tests
cargo test --test utils_test
```

### Run unit tests only
```bash
cargo test --lib
```

### Run integration tests only
```bash
cargo test --test '*'
```

## Writing Tests

Tests use `mockito` to mock HTTP requests. Each test follows this pattern:

1. Create a mock server
2. Set up expected request/response
3. Create a client and call the API
4. Assert the result
5. Verify the mock was called

Example:
```rust
#[tokio::test]
async fn test_example() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/api/v1/endpoint")
        .with_status(200)
        .with_body(r#"{"data": "test"}"#)
        .create();

    let client = Client::with_base_url("test-key".to_string(), server.url()).unwrap();
    let result = client.some_api().some_method().await;

    assert!(result.is_ok());
    mock.assert();
}
```

## Notes

- Tests do not call the real API
- HTTP requests are simulated using a mock server
- Each test runs independently
