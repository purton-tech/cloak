use crate::errors::CustomError;
use axum::response::Html;

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
pub fn layout(title: &str, content: &str) -> Result<Html<String>, CustomError> {
    let html = ApplicationLayout { content, title };

    Ok(Html(html.to_string().replace("sl_drawer", "sl-drawer")))
}

markup::define! {
    ApplicationLayout<'a>(content: &'a str, title: &'a str)
    {
        {markup::doctype()}
        html {

            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { {title} }

                script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}

                link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}

            }

            body {
                div.l_application {
                    header {
                    }
                    aside.sidenav {
                        h1 {
                            a[href=crate::vaults::INDEX] { "KeyVault" }
                        }
                        h2 {
                            "Your Vaults"
                        }
                        ul {
                            li.selected {
                                img[src=crate::statics::get_vault_svg(), width="32px", type="image/svg"] {}
                                a[href=crate::vaults::INDEX] { "Vaults" }
                            }
                        }
                    }
                    main.container {
                        section.content {
                            div {
                                {markup::raw(content)}
                            }
                        }
                    }
                }
            }
        }
    }
}
