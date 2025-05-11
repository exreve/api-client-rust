# API Client for rust

This is an auto-generated SDK for the API.

## Example Usage

```rs
// DISCLAIMER: This is an auto-generated example and library.
// This lib has NOT been tested by our support team.
// Please use at your own risk, and make tests before using it.
//
// install using "cargo install exkoin-api-client"
use exkoin_api_client::{apis::configuration::Configuration, apis::default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut configuration = Configuration::new();
    configuration.base_path = "https://api.exkoin.com".to_string();
    configuration.api_key = Some(
        "ApiKeyAuth".to_string(),
        "YOUR_API_KEY".to_string()
    ).into();
    configuration.api_secret = Some(
        "ApiSecretAuth".to_string(),
        "YOUR_API_SECRET".to_string()
    ).into();

    let api = default_api::DefaultApi::new(configuration);
    
    match api.get_balance().await {
        Ok(balance) => println!("Account balance: {:?}", balance),
        Err(e) => eprintln!("Error fetching balance: {:?}", e),
    }
    
    Ok(())
}
```