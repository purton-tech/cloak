use crate::errors::CustomError;
use crate::vault::{vault_client::VaultClient, ListVaultsRequest, ListVaultsResponse};
use actix_web::{web, HttpResponse};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

pub async fn index(
    config: web::Data<crate::config::Config>,
    auth: crate::authentication::Authentication,
) -> Result<HttpResponse, CustomError> {
    let channel = Channel::builder(config.vault_server_uri.clone())
        .connect()
        .await?;

    let token = MetadataValue::from_str(&auth.user_id.to_string())?;

    let mut client = VaultClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("x-user-id", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(ListVaultsRequest {});

    let vaults = client.list_vaults(request).await?;

    let page = VaultsPage {
        vaults: vaults.into_inner(),
    };

    crate::layout::layout("Home", &page.to_string())
}

markup::define! {
    VaultsPage(vaults: ListVaultsResponse) {
        div.m_card {
            div.header {
                span { "Vaults" }

                sl_drawer[label="Add Vault", class="add-vault"] {
                    p {
                        "Folders keep related secrets together.
                        For example you could have a folder called Database with all
                        the secrets related to database access."
                    }

                    form.m_form[style="margin-top: 2em", method = "post", action=super::NEW] {
                        fieldset {
                            label[for="name"] { "Name" }
                            input[type="text", required="", name="name"] {}
                        }
                        button.a_button.auto.success[slot="footer", type = "submit"] { "Create Vault" }
                    }
                    button[class="a_button", slot="footer", type="primary"] { "Close" }
                }

                button.a_button.mini.primary."drawer-opener" { "Add Vault" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Items" }
                        }
                    }
                    tbody {
                        @for vault in &vaults.vaults {
                            tr {
                                td { {vault.name} }
                                td { "Updated" }
                                td { "Created" }
                                td { "Items" }
                            }
                        }
                    }
                }
            }
        }
    }
}
