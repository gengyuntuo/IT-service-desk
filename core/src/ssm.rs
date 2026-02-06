use aws_config::BehaviorVersion;
use aws_sdk_ssm::{Client, Error};

// Lists your parameters.
// snippet-start:[ssm.rust.describe-parameters]
async fn show_parameters(client: &Client) -> Result<(), Error> {
    let resp = client.describe_parameters().send().await?;

    for param in resp.parameters() {
        println!("  {}", param.name().unwrap_or_default());
    }

    Ok(())
}

async fn get_ssm_client() -> Result<Client, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::v2026_01_12()).await;
    Ok(Client::new(&config))
}
