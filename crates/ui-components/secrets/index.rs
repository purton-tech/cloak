use crate::cloak_layout::{CloakLayout, SideBar};
use db::{Environment, Secret, UserVault};
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
struct SecretProps {
    user_vault: UserVault,
    environments: Vec<Environment>,
    secrets: Vec<Secret>,
    organisation_id: i32,
}

pub fn index(
    organisation_id: i32,
    user_vault: UserVault,
    environments: Vec<Environment>,
    secrets: Vec<Secret>,
) -> String {
    fn app(cx: Scope<SecretProps>) -> Element {
        cx.render(rsx! {

            CloakLayout {
                selected_item: SideBar::Secrets,
                team_id: cx.props.organisation_id,
                title: "Secrets",
                vault_id: cx.props.user_vault.vault_id
                header: cx.render(rsx!(
                    h3 { "Secrets" }
                ))
                div {

                }
            }
        })
    }

    let mut app = VirtualDom::new_with_props(
        app,
        SecretProps {
            organisation_id,
            user_vault,
            environments,
            secrets,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
