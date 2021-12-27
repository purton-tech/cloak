pub mod vault {
    tonic::include_proto!("vault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut client = vault_client::VaultClient::connect("https://keyvault.authn.tech").await?;
    let client = vault::vault_client::Vault::new(String::from("http://envoy:7100"));

    let response = client.list_vaults(vault::ListVaultsRequest {}).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
