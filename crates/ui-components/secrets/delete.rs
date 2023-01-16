#![allow(non_snake_case)]
use db::{Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct DrawerProps<'a> {
    organisation_id: i32,
    secret: &'a Secret,
    user_vault: &'a UserVault,
    trigger_id: String,
}

pub fn DeleteSecretDrawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::secrets::delete_route(cx.props.secret.vault_id, cx.props.organisation_id),
            label: "Delete Secret?",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to delete the secret "
                        }
                        strong {
                            {LazyNodes::new(|f| f.text(format_args!(
                                "<ecdh-cipher cipher='{}'
                                wrapped-aes-key='{}' 
                                ecdh-public-key='{}'></ecdh-cipher>",
                                cx.props.secret.name,
                                cx.props.user_vault.encrypted_vault_key,
                                cx.props.user_vault.ecdh_public_key
                            )))}
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "secret_id",
                        "value": "{cx.props.secret.id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Delete Secret"
                }
            }
        }
    })
}
