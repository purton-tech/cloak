use crate::cloak_layout::{CloakLayout, SideBar};
use db::{EnvironmentsAndVault, ServiceAccount};
use dioxus::prelude::*;

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
                ))
                div {

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
