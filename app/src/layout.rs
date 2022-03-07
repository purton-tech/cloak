use crate::errors::CustomError;
use axum::{http::Response, response::Html};
use hyper::{Body, StatusCode};

#[derive(PartialEq, Eq)]
pub enum SideBar {
    Vaults,
    Secrets,
    Members,
    ServiceAccounts,
    Team,
}

pub fn redirect_and_snackbar(
    url: &str,
    message: &'static str,
) -> Result<Response<Body>, CustomError> {
    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", url)
        .header("set-cookie", format!("flash_aargh={}; Max-Age=6", message))
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
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
            .replace("<new_vault", "<new-vault")
            .replace("</new_vault", "</new-vault")
            .replace("<snack_bar", "<snack-bar")
            .replace("</snack_bar", "</snack-bar")
            .replace("</new_account", "</new-account")
            .replace("<new_account", "<new-account")
            .replace("</connect_account", "</connect-account")
            .replace("<connect_account", "<connect-account")
            .replace("</view_account", "</view-account")
            .replace("<view_account", "<view-account")
            .replace("<new_secret", "<new-secret")
            .replace("</new_secret", "</new-secret")
            .replace("ecdh_cipher", "ecdh-cipher")
            .replace("relative_time", "relative-time"),
    ))
}

fn get_menu_class(side_bar: &SideBar, selected_sidebar: &SideBar, sub_menu: bool) -> String {
    let selected = selected_sidebar == side_bar;
    match (selected, sub_menu) {
        (true, true) => "selected submenu",
        (true, false) => "selected",
        (false, true) => "submenu",
        (false, false) => "",
    }
    .to_string()
}

markup::define! {

    SvgSideMenuItem<'a>(side_bar: SideBar, name: &'a str, link: &'a str,
        svg: &'a str, selected_sidebar: &'a SideBar, sub_menu: bool) {

        li[class={get_menu_class(side_bar, selected_sidebar, *sub_menu)}] {
            img[alt="Menu Item", width = "24px", src = svg] { }
            a[href=link] { {name} }
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
                        ul {

                            { SvgSideMenuItem { side_bar: SideBar::Vaults, name: "Vaults",
                                link: crate::vaults::INDEX,
                                svg: &crate::statics::get_vault_svg(),
                                selected_sidebar: side_bar, sub_menu: false  } }

                            @if let Some(vault_id) = vault {
                                { SvgSideMenuItem { side_bar: SideBar::Secrets, name: "Secrets",
                                    link: &crate::secrets::secret_route(*vault_id as i32),
                                    svg: &crate::statics::get_secrets_svg(),
                                    selected_sidebar: side_bar, sub_menu: true  } }
                                { SvgSideMenuItem { side_bar: SideBar::Members, name: "Members",
                                    link: &crate::members::member_route(*vault_id),
                                    svg: &crate::statics::get_users_svg(),
                                    selected_sidebar: side_bar, sub_menu: true  } }
                            }

                            { SvgSideMenuItem { side_bar: SideBar::ServiceAccounts, name: "Service Accounts",
                                link: crate::service_accounts::INDEX,
                                svg: &crate::statics::get_accounts_svg(),
                                selected_sidebar: side_bar, sub_menu: false  } }

                            { SvgSideMenuItem { side_bar: SideBar::Team, name: "Team",
                                link: crate::team::INDEX,
                                svg: &crate::statics::get_users_svg(),
                                selected_sidebar: side_bar, sub_menu: false  } }
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
                snack_bar {}
            }
        }
    }
}
