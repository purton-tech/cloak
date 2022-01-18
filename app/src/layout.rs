use crate::errors::CustomError;
use axum::response::Html;

#[derive(PartialEq, Eq)]
pub enum SideBar {
    Vaults,
    ServiceAccounts,
}

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
pub fn layout(title: &str, content: &str, side_bar: &SideBar) -> Result<Html<String>, CustomError> {
    let html = ApplicationLayout {
        content,
        title,
        side_bar,
    };

    Ok(Html(html.to_string().replace("sl_drawer", "sl-drawer")))
}

markup::define! {

    SvgSideMenuItem<'a>(side_bar: SideBar, name: &'a str, link: &'a str,
        svg: &'a str, selected_sidebar: &'a SideBar) {
        @if *selected_sidebar == side_bar {
            li.selected {
                img[alt="Satellite", width = "24px", src = svg] { }
                a[href=link] { {name} }
            }
        } else {
            li {
                img[alt="Satellite", width = "24px", src = svg] { }
                a[href=link] { {name} }
            }
        }
    }

    ApplicationLayout<'a>(content: &'a str, title: &'a str, side_bar: &'a SideBar)
    {
        @markup::doctype()

        html[lang="en"] {

            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { {title} }

                script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}

                link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}

                // default-src 'self'           Only load resources from our server.
                // style-src 'unsafe-inline'    Shoelace style inserts styling into the dom.
                // connect-src data: =>         Allow the inline svg for sl-drawer
                // connect-src 'self' =>        Allow connections back to the server so Web gRPC works
                meta ["http-equiv"="Content-Security-Policy",
                    content="default-src 'self'; style-src 'unsafe-inline'; connect-src 'self' data:"] {}

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

                            { SvgSideMenuItem { side_bar: SideBar::Vaults, name: "Vaults",
                                link: crate::vaults::INDEX,
                                svg: &crate::statics::get_vault_svg(), selected_sidebar: side_bar  } }


                            { SvgSideMenuItem { side_bar: SideBar::ServiceAccounts, name: "Service Accounts",
                                link: crate::service_accounts::INDEX,
                                svg: &crate::statics::get_vault_svg(), selected_sidebar: side_bar  } }
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
