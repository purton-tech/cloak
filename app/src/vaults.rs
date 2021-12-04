use crate::errors::CustomError;
use actix_web::{web, HttpResponse};
use horrorshow::owned_html;

pub static INDEX: &str = "/app/vaults";

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource(INDEX).route(web::get().to(index)));
}

pub async fn index() -> Result<HttpResponse, CustomError> {
    let page = owned_html! {
        div(class="m_card") {
            div(class="header") {
                span { :"Vaults" }

                sl-drawer(label="Add Vault", class="add-vault") {
                    p {
                        :"Folders keep related secrets together.
                        For example you could have a folder called Database with all
                        the secrets related to database access."
                    }

                    form(class="m_form", style="margin-top: 2em") {
                        fieldset {
                            label(for="name") { :"Name" }
                        }
                    }
                    button(class="a_button", slot="footer", type="primary") { :"Close" }
                }

                button(class="a_button mini primary drawer-opener") { :"Add Vault" }
            }
            div(class="body") {
                table(class="m_table") {
                    thead {
                        tr {
                            th { :"Name" }
                            th { :"Updated" }
                            th { :"Created" }
                            th { :"Items" }
                        }
                    }
                }
            }
        }
    };

    Ok(crate::layout::layout("Home", page)?)
}
