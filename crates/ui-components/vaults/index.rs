use crate::cloak_layout::{CloakLayout, SideBar};
use crate::routes::secrets::index_route;
use assets::files::{button_plus_svg, empty_api_keys_svg};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(PartialEq, Eq)]
pub struct VaultSummary {
    pub id: i32,
    pub name: String,
    pub user_count: i32,
    pub secrets_count: i32,
    pub href: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Props, PartialEq)]
struct VaultProps {
    organisation_id: i32,
    vaults: Vec<VaultSummary>,
}

pub fn index(organisation_id: i32, vaults: Vec<VaultSummary>) -> String {
    fn app(cx: Scope<VaultProps>) -> Element {
        cx.render(rsx! {

            CloakLayout {
                selected_item: SideBar::Vaults,
                team_id: cx.props.organisation_id,
                title: "Vaults"
                header: cx.render(rsx!(
                    h3 { "Vaults" }

                    if ! cx.props.vaults.is_empty() {
                        cx.render(rsx! {
                            Button {
                                prefix_image_src: "{button_plus_svg.name}",
                                drawer_trigger: super::new_vault::DRAW_TRIGGER,
                                button_scheme: ButtonScheme::Primary,
                                "Create A New Vault"
                            }
                        })
                    } else {
                        None
                    }
                ))

                if cx.props.vaults.is_empty() {
                    cx.render(rsx! {
                        BlankSlate {
                            heading: "You don't have any vaults yet",
                            visual: empty_api_keys_svg.name,
                            description: "Vaults allow you to keep related secrets together.",
                            primary_action_drawer: ("Create A New Vault", super::new_vault::DRAW_TRIGGER)
                        }
                    })
                } else {
                    cx.render(rsx! {
                        super::table::VaultTable {
                            vaults: &cx.props.vaults,
                            organisation_id: cx.props.organisation_id
                        }
                    })
                }

                super::new_vault::NewVaultForm {
                    submit_action: crate::routes::vaults::new_route(cx.props.organisation_id)
                }
            }
        })
    }

    let vaults = vaults
        .into_iter()
        .map(|v| VaultSummary {
            href: index_route(v.id, organisation_id),
            ..v
        })
        .collect();

    let mut app = VirtualDom::new_with_props(
        app,
        VaultProps {
            organisation_id,
            vaults,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
