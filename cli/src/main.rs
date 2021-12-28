pub mod vault {
    tonic::include_proto!("vault");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = vault::vault_client::Vault::new(String::from("https://keyvault.authn.tech"));
    //let client = vault::vault_client::Vault::new(String::from("http://envoy:7100"));

    let response = client.list_vaults(vault::ListVaultsRequest {}).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
