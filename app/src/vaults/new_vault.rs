use crate::errors::CustomError;
use crate::vault::vault_client::VaultClient;
use crate::vault::VaultRequest;
use actix_web::{http, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}

pub async fn new(
    form: web::Form<NewVault>,
    config: web::Data<crate::config::Config>,
) -> Result<HttpResponse, CustomError> {
    let mut client = VaultClient::connect(config.vault_server_url.clone()).await?;

    let request = tonic::Request::new(VaultRequest {
        name: form.name.clone(),
    });

    let response = client.create_vault(request).await?;

    println!("RESPONSE={:?}", response);
    dbg!(&form);

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, super::INDEX))
        .finish())
}
