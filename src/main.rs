use monoova_rs_sdk::{Client, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with API key
    let client = Client::with_api_key_and_env(
        "your-api-key-here".to_string(),
        Environment::Sandbox, // Use Sandbox for testing
    )?;

    // Example: Get account balance
    match client.financial().get_balance().await {
        Ok(balance) => {
            println!("Account Balance: {:.2} {}", balance.balance, balance.currency);
            println!("Available Balance: {:.2} {}", balance.available_balance, balance.currency);
        }
        Err(e) => {
            eprintln!("Error getting balance: {}", e);
        }
    }

    Ok(())
}
