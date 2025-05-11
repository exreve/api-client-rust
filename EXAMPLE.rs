
use exkoin_api_client::{apis::configuration::Configuration, apis::default_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut configuration = Configuration::new();
    configuration.base_path = "https://api.exkoin.com".to_string();
    configuration.api_key = Some(
        "ApiKeyAuth".to_string(),
        "YOUR_API_KEY".to_string()
    ).into();

    let api = default_api::DefaultApi::new(configuration);
    
    match api.get_balance().await {
        Ok(balance) => println!("Account balance: {:?}", balance),
        Err(e) => eprintln!("Error fetching balance: {:?}", e),
    }
    
    Ok(())
}
      