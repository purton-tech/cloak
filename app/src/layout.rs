use crate::errors::CustomError;
use axum::response::Html;

#[derive(PartialEq, Eq)]
pub enum SideBar {
    Vaults,
    ServiceAccounts,
    Team,
}

// page_title and content can be anything that can be rendered. A string, a
// template, a number, etc.
pub fn layout(title: &str, content: &str, side_bar: &SideBar) -> Result<Html<String>, CustomError> {
    layout_with_header(title, content, "", side_bar)
}

pub fn layout_with_header(
    title: &str,
    content: &str,
    header: &str,
    side_bar: &SideBar,
) -> Result<Html<String>, CustomError> {
    vault_layout(title, content, header, side_bar, None)
}

pub fn vault_layout(
    title: &str,
    content: &str,
    header: &str,
    side_bar: &SideBar,
    vault: Option<u32>,
) -> Result<Html<String>, CustomError> {
    let html = ApplicationLayout {
        content,
        header,
        title,
        side_bar,
        vault,
    };

    Ok(Html(
        html.to_string()
            .replace("side_drawer", "side-drawer")
            .replace("invite_user", "invite-user")
            .replace("add_member", "add-member")
            .replace("relative_time", "relative-time"),
    ))
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

    ApplicationLayout<'a>(
        content: &'a str,
        header: &'a str,
        title: &'a str,
        side_bar: &'a SideBar,
        vault: Option<u32>
    )
    {
        @markup::doctype()

        html[lang="en"] {

            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { {title} }

                script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}


                link [ rel="icon", type="image/svg+xml", href=crate::statics::get_favicon_svg()] {}

                link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}
            }

            body {
                div.l_application {
                    header {
                        {markup::raw(header)}
                    }
                    aside.sidenav {
                        h1 {
                            a[href=crate::vaults::INDEX] { "Cloak" }
                        }
                        h2 {
                            "Your Vaults"
                        }
                        ul {

                            { SvgSideMenuItem { side_bar: SideBar::Vaults, name: "Vaults",
                                link: crate::vaults::INDEX,
                                svg: &crate::statics::get_vault_svg(), selected_sidebar: side_bar  } }

                            @if let Some(vault_id) = vault {
                                { SvgSideMenuItem { side_bar: SideBar::Vaults, name: "Secrets",
                                    link: &crate::secrets::secret_route(*vault_id as i32),
                                    svg: &crate::statics::get_vault_svg(), selected_sidebar: side_bar  } }
                                { SvgSideMenuItem { side_bar: SideBar::Vaults, name: "Members",
                                    link: &crate::members::member_route(*vault_id as i32),
                                    svg: &crate::statics::get_vault_svg(), selected_sidebar: side_bar  } }
                            }

                            { SvgSideMenuItem { side_bar: SideBar::ServiceAccounts, name: "Service Accounts",
                                link: crate::service_accounts::INDEX,
                                svg: &crate::statics::get_vault_svg(), selected_sidebar: side_bar  } }

                            { SvgSideMenuItem { side_bar: SideBar::Team, name: "Team",
                                link: crate::team::INDEX,
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
