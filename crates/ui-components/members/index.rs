use crate::cloak_layout::{CloakLayout, SideBar};
use assets::files::button_plus_svg;
use db::{Environment, NonMember, UserVault, VaultMember};
use dioxus::prelude::*;
use primer_rsx::*;

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
                    if ! cx.props.non_members.is_empty() {
                        cx.render(rsx! {
                            Button {
                                prefix_image_src: "{button_plus_svg.name}",
                                drawer_trigger: super::add_member::DRAW_TRIGGER,
                                button_scheme: ButtonScheme::Primary,
                                "Add Member"
                            }
                        })
                    } else {
                        None
                    }
                ))
                super::table::MembersTable {
                    members: cx.props.members.clone(),
                }
            }
            super::add_member::AddMemberForm {
                user_vault: cx.props.user_vault.clone(),
                submit_action: crate::routes::members::add_route(
                    cx.props.user_vault.vault_id,
                    cx.props.organisation_id),
                non_members: cx.props.non_members.clone(),
                environments: cx.props.environments.clone(),
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
