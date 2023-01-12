use crate::cloak_layout::{CloakLayout, SideBar};
use assets::files::button_plus_svg;
use assets::files::empty_api_keys_svg;
use db::{EnvironmentsAndVault, ServiceAccount};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
struct SAProps {
    organisation_id: i32,
    service_accounts: Vec<ServiceAccount>,
    environments_and_vaults: Vec<EnvironmentsAndVault>,
}

pub fn index(
    organisation_id: i32,
    service_accounts: Vec<ServiceAccount>,
    environments_and_vaults: Vec<EnvironmentsAndVault>,
) -> String {
    fn app(cx: Scope<SAProps>) -> Element {
        cx.render(rsx! {

            CloakLayout {
                selected_item: SideBar::ServiceAccounts,
                team_id: cx.props.organisation_id,
                title: "Service Accounts"
                header: cx.render(rsx!(
                    h3 { "Service Accounts" }

                    if ! cx.props.service_accounts.is_empty() {
                        cx.render(rsx! {
                            Button {
                                prefix_image_src: "{button_plus_svg.name}",
                                drawer_trigger: super::new_account::DRAW_TRIGGER,
                                button_scheme: ButtonScheme::Primary,
                                "Add Service Account"
                            }
                        })
                    } else {
                        None
                    }
                ))

                if cx.props.service_accounts.is_empty() {
                    cx.render(rsx! {
                        BlankSlate {
                            heading: "You don't have any service accounts yet",
                            visual: empty_api_keys_svg.name,
                            description: "To allow applications to access secrets without human intervention,
                            We support service accounts. A service account is a non-human account 
                            that is tied to one or more vaults.",
                            primary_action_drawer: ("Add Service Account", super::new_account::DRAW_TRIGGER)
                        }
                    })
                } else {
                    cx.render(rsx! {
                        super::table::ServiceAccountTable {
                            service_accounts: cx.props.service_accounts.clone()
                        }
                    })
                }

                super::new_account::NewAccountForm {
                    submit_action: crate::routes::service_accounts::new_route(
                        cx.props.organisation_id),
                }
            }
        })
    }

    let mut app = VirtualDom::new_with_props(
        app,
        SAProps {
            organisation_id,
            service_accounts,
            environments_and_vaults,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
