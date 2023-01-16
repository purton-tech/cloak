#![allow(non_snake_case)]
use db::VaultMember;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct DrawerProps<'a> {
    organisation_id: i32,
    vault_member: &'a VaultMember,
    trigger_id: String,
}

pub fn RemoveMemberDrawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::members::delete_route(
                cx.props.vault_member.vault_id, cx.props.organisation_id),
            label: "Remove Member from Vault ?",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to remove {cx.props.vault_member.email}?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "user_id",
                        "value": "{cx.props.vault_member.user_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "vault_id",
                        "value": "{cx.props.vault_member.vault_id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Remove Member"
                }
            }
        }
    })
}
