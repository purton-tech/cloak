use crate::cloak_layout::{CloakLayout, SideBar};
use db::{Environment, Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

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
        if cx.props.secrets.is_empty() {
            cx.render(rsx! {
                super::empty::EmptySecrets {
                    organisation_id: cx.props.organisation_id
                }
            })
        } else {
            cx.render(rsx! {

                CloakLayout {
                    selected_item: SideBar::Secrets,
                    team_id: cx.props.organisation_id,
                    title: "Secrets",
                    vault_id: cx.props.user_vault.vault_id
                    header: cx.render(rsx!(
                        h3 { "Secrets" }
                    ))
                    TabContainer {
                        tabs: cx.render(rsx! {
                            cx.props.environments.iter().map(|env| rsx!(
                                TabHeader {
                                    selected: false,
                                    tab: "{env.name}-panel",
                                    name: &env.name
                                }
                            ))
                        })
                        cx.props.environments.iter().map(|env| rsx!(
                            TabPanel {
                                hidden: true,
                                id: "{env.name}-panel",
                                div {
                                    h4 { "{env.name}" }
                                }
                            }
                        ))
                    }
                }
            })
        }
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
