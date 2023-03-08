use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{output::ListTablesOutput, Client, Error};

/// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider: RegionProviderChain =
        RegionProviderChain::default_provider().or_else("us-east-1");
    let config: aws_config::SdkConfig = aws_config::from_env().region(region_provider).load().await;
    let client: Client = Client::new(&config);

    let resp: ListTablesOutput = client.list_tables().send().await?;

    println!("Tables:");

    let names: &[String] = resp.table_names().unwrap_or_default();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}
