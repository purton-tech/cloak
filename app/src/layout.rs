use crate::errors::CustomError;
use actix_web::HttpResponse;
use horrorshow::owned_html;
use horrorshow::prelude::*;

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
pub fn layout(page_title: impl Render, content: impl Render) -> Result<HttpResponse, CustomError> {
    // owned_html _moves_ the arguments into the template. Useful for returning
    // owned (movable) templates.
    let html = owned_html! {
        html {
            head {
                title : &page_title;
                meta(charset="utf-8") {}
                meta(http-equiv="X-UA-Compatible", content="IE=edge") {}
                meta(name="viewport", content="width=device-width, initial-scale=1") {}
                // Only load sources from self.
                //meta(http-equiv="Content-Security-Policy", content="default-src 'self'") {}
                script(src = crate::statics::get_index_js(), type="text/javascript", async="") {}
                link(rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()) {}

            }
            body {
                div(class="l_application") {
                    header {
                    }
                    aside {
                        h1 {
                            a(href=crate::vaults::INDEX) { :"KeyVault" }
                        }
                        ul {
                            li(class="selected") {
                                img(src=crate::statics::get_vault_svg(), width="32px", type="image/svg") {}
                                a(href=crate::vaults::INDEX) { :"Vaults" }
                            }
                        }
                    }
                    main(class="container") {
                        section(class="content") {
                            :&content
                        }
                    }
                }
            }
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html.into_string()?))
}
