#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

pub static DRAW_TRIGGER: &str = "new-vault-drawer";

#[derive(Props, PartialEq, Eq)]
pub struct NewVaultFormProps {
    submit_action: String,
}

pub fn NewVaultForm(cx: Scope<NewVaultFormProps>) -> Element {
    cx.render(rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "Add Vault",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
                        Alert {
                            alert_color: AlertColor::Success,
                            class: "mb-3",
                            "Vaults keep related secrets together.
                            For example you could have a vault called My Project with all
                            the secrets related to your project."
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "Give your vault a name"
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        {LazyNodes::new(|f| f.text(format_args!(
                            "<ecdh-keygen public='public_key',
                                private='encrypted_vault_key'></ecdh-keygen>")))}
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create"
                    }
                }
            }
        }
    })
}
