use crate::errors::CustomError;
use axum::response::Html;

pub async fn index(//config: web::Data<crate::config::Config>,
    //auth: crate::authentication::Authentication,
) -> Result<Html<String>, CustomError> {
    let page = VaultsPage {};

    crate::layout::layout("Home", &page.to_string())
}

markup::define! {
    VaultsPage {
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
                    }
                }
            }
        }
    }
}
