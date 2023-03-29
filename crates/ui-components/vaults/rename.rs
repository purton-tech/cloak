#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps<'a> {
    organisation_id: i32,
    vault: &'a super::index::VaultSummary,
    trigger_id: String,
}

pub fn RenameVaultDrawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::vaults::rename_route(cx.props.organisation_id),
            label: "Rename Vault",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Input {
                        input_type: InputType::Text,
                        help_text: "Please confirm the new name of the vault"
                        required: true,
                        label: "Name",
                        value: &cx.props.vault.name,
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
                    button_scheme: ButtonScheme::Primary,
                    "Rename Vault"
                }
            }
        }
    })
}
