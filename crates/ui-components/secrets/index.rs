use crate::cloak_layout::{CloakLayout, SideBar};
use assets::files::button_plus_svg;
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
                    organisation_id: cx.props.organisation_id,
                    vault_id: cx.props.user_vault.vault_id
                }
                super::new_secret::NewSecretForm {
                    submit_action: crate::routes::secrets::new_route(
                        cx.props.user_vault.vault_id,
                        cx.props.organisation_id),
                    user_vault: cx.props.user_vault.clone(),
                    environments: cx.props.environments.clone()
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
                        Button {
                            prefix_image_src: "{button_plus_svg.name}",
                            drawer_trigger: super::new_secret::DRAW_TRIGGER,
                            button_scheme: ButtonScheme::Primary,
                            "Create A New Secret"
                        }
                    ))
                    TabContainer {
                        tabs: cx.render(rsx! {
                            cx.props.environments.iter().enumerate().map(|(index, env)| rsx!(
                                TabHeader {
                                    selected: index == 0,
                                    tab: "{env.name}-panel",
                                    name: &env.name
                                }
                            ))
                        })
                        cx.props.environments.clone().into_iter().enumerate().map(|(index, env)| rsx!(
                            TabPanel {
                                hidden: index != 0,
                                id: "{env.name}-panel",
                                super::table::SecretsTable {
                                    user_vault: cx.props.user_vault.clone(),
                                    secrets: cx.props.secrets.clone(),
                                    environment: env
                                }
                            }
                        ))
                    }
                }
                super::new_secret::NewSecretForm {
                    submit_action: crate::routes::secrets::new_route(
                        cx.props.user_vault.vault_id,
                        cx.props.organisation_id),
                    user_vault: cx.props.user_vault.clone(),
                    environments: cx.props.environments.clone()
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