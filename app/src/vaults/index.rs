use crate::errors::CustomError;
use actix_web::HttpResponse;
use horrorshow::owned_html;

pub async fn index() -> Result<HttpResponse, CustomError> {
    let page = owned_html! {
        div(class="m_card") {
            div(class="header") {
                span { :"Vaults" }

                form(class="m_form", method = "post", action=super::NEW) {
                    sl-drawer(label="Add Vault", class="add-vault") {
                        p {
                            :"Folders keep related secrets together.
                            For example you could have a folder called Database with all
                            the secrets related to database access."
                        }

                        form(class="m_form", style="margin-top: 2em") {
                            fieldset {
                                label(for="name") { :"Name" }
                                input(type="text", required="", name="name") {}
                            }
                        }
                        button(class="a_button auto success", slot="footer", type = "submit") { :"Create Vault" }
                    }
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
