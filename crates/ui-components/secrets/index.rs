use crate::cloak_layout::{CloakLayout, SideBar};
use assets::files::button_plus_svg;
use db::{Environment, Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

pub static NEW_SECRET_DRAW_TRIGGER: &str = "add-secret-drawer";

#[derive(Props, PartialEq)]
struct SecretProps {
    organisation_id: i32,
    user_vault: UserVault,
    env_secrets: Vec<(Environment, Vec<Secret>)>,
    secrets: Vec<Secret>,
    environments: Vec<Environment>,
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
                super::form::SecretForm {
                    submit_action: crate::routes::secrets::new_route(
                        cx.props.user_vault.vault_id,
                        cx.props.organisation_id),
                    user_vault: &cx.props.user_vault,
                    environments: &cx.props.environments,
                    trigger_id: NEW_SECRET_DRAW_TRIGGER.to_string()
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
                            drawer_trigger: NEW_SECRET_DRAW_TRIGGER,
                            button_scheme: ButtonScheme::Primary,
                            "Create A New Secret"
                        }
                    ))
                    TabContainer {
                        tabs: cx.render(rsx! {
                            TabHeader {
                                selected: true,
                                tab: "all-panel",
                                name: "All"
                            }
                            cx.props.environments.iter().map(|env| rsx!(
                                TabHeader {
                                    selected: false,
                                    tab: "{env.name}-panel",
                                    name: &env.name
                                }
                            ))
                        })
                        TabPanel {
                            hidden: false,
                            id: "all-panel",
                            super::table::SecretsTable {
                                user_vault: cx.props.user_vault.clone(),
                                secrets: cx.props.secrets.clone(),
                                environments: cx.props.environments.clone(),
                                organisation_id: cx.props.organisation_id
                            }
                        }
                        cx.props.env_secrets.clone().into_iter().map(|env_secrets| rsx!(
                            TabPanel {
                                hidden: true,
                                id: "{env_secrets.0.name}-panel",
                                super::table::SecretsTable {
                                    user_vault: cx.props.user_vault.clone(),
                                    secrets: env_secrets.1.clone(),
                                    organisation_id: cx.props.organisation_id,
                                    environments: cx.props.environments.clone(),
                                    environment: env_secrets.0
                                }
                            }
                        ))
                    }
                }
                super::form::SecretForm {
                    submit_action: crate::routes::secrets::new_route(
                        cx.props.user_vault.vault_id,
                        cx.props.organisation_id),
                    user_vault: &cx.props.user_vault,
                    environments: &cx.props.environments,
                    trigger_id: NEW_SECRET_DRAW_TRIGGER.to_string()
                }
            })
        }
    }

    // Filter the secrets by environement
    let env_secrets: Vec<(Environment, Vec<Secret>)> = environments
        .clone()
        .into_iter()
        .map(|env| {
            let sec = secrets
                .clone()
                .into_iter()
                .filter(|s| s.environment_id == env.id)
                .collect();
            (env, sec)
        })
        .collect();

    let mut app = VirtualDom::new_with_props(
        app,
        SecretProps {
            organisation_id,
            user_vault,
            env_secrets,
            secrets,
            environments,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
