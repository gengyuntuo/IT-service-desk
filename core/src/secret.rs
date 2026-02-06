use aws_sdk_secretsmanager::config::BehaviorVersion;
use aws_sdk_secretsmanager::{Client, Error};

// Displays the value of a secret.
// snippet-start:[secretsmanager.rust.get-secret-value]
async fn show_secret(client: &Client, name: &str) -> Result<(), Error> {
    let resp = client.get_secret_value().secret_id(name).send().await?;

    println!("Value: {}", resp.secret_string().unwrap_or("No value!"));

    Ok(())
}

async fn get_secret_client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2026_01_12()).await;
    Ok(Client::new(&config))
}
