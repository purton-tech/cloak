use crate::cloak_layout::{CloakLayout, SideBar};
use crate::routes::secrets::index_route;
use assets::files::button_plus_svg;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(PartialEq, Eq)]
pub struct VaultSummary {
    pub id: i32,
    pub name: String,
    pub user_count: i32,
    pub secrets_count: i32,
    pub href: String,
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
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        drawer_trigger: super::new_vault::DRAW_TRIGGER,
                        button_scheme: ButtonScheme::Primary,
                        "Create A New Vault"
                    }
                ))

                Box {
                    BoxHeader {
                        title: "Your Vaults"
                    }
                    BoxBody {
                        DataTable {
                            table {
                                thead {
                                    th { "Name" }
                                    th { "Created" }
                                    th { "Members" }
                                    th { "Secrets" }
                                }
                                tbody {
                                    cx.props.vaults.iter().map(|vault| rsx!(
                                        tr {
                                            td {
                                                a {
                                                    href: "{vault.href}",
                                                    "{vault.name}"
                                                }
                                            }
                                            td {
                                                "{vault.name}"
                                            }
                                            td {
                                                "{vault.user_count}"
                                            }
                                            td {
                                                "{vault.secrets_count}"
                                            }
                                        }
                                    ))
                                }
                            }
                        }
                    }
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
