use app::vault::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = vault_client::VaultClient::connect("https://keyvault.authn.tech").await?;

    let request = tonic::Request::new(ListVaultsRequest {});

    let response = client.list_vaults(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
