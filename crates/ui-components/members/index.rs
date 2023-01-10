use crate::cloak_layout::{CloakLayout, SideBar};
use db::{Environment, NonMember, UserVault, VaultMember};
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
struct MemberProps {
    user_vault: UserVault,
    environments: Vec<Environment>,
    members: Vec<VaultMember>,
    non_members: Vec<NonMember>,
    organisation_id: i32,
}

pub fn index(
    organisation_id: i32,
    user_vault: UserVault,
    environments: Vec<Environment>,
    members: Vec<VaultMember>,
    non_members: Vec<NonMember>,
) -> String {
    fn app(cx: Scope<MemberProps>) -> Element {
        cx.render(rsx! {

            CloakLayout {
                selected_item: SideBar::Members,
                team_id: cx.props.organisation_id,
                title: "Members",
                vault_id: cx.props.user_vault.vault_id
                header: cx.render(rsx!(
                    h3 { "Members" }
                ))
                div {

                }
            }
        })
    }

    let mut app = VirtualDom::new_with_props(
        app,
        MemberProps {
            organisation_id,
            user_vault,
            environments,
            members,
            non_members,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
