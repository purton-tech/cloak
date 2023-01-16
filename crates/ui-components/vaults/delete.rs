#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps<'a> {
    organisation_id: i32,
    vault: &'a super::index::VaultSummary,
    trigger_id: String,
}

pub fn DeleteVaultDrawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::vaults::delete_route(cx.props.organisation_id),
            label: "Delete Vault ?",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to delete this vault?"
                        }
                        "If so then type the name of the vault "
                        strong {
                            "{cx.props.vault.name}"
                        }
                        "into the input field"
                    }
                    Input {
                        input_type: InputType::Text,
                        help_text: "Please confirm the name of the vault you wish to delete"
                        required: true,
                        label: "Name",
                        name: "name"
                    }
                    input {
                        "type": "hidden",
                        "name": "vault_id",
                        "value": "{cx.props.vault.id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Delete Vault"
                }
            }
        }
    })
}
