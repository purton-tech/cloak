#![allow(non_snake_case)]
#![allow(unused_braces)]
use db::{Environment, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

pub static DRAW_TRIGGER: &str = "add-secret-drawer";

#[derive(Props, PartialEq)]
pub struct NewSecretFormProps {
    submit_action: String,
    user_vault: UserVault,
    environments: Vec<Environment>,
}

pub fn NewSecretForm(cx: Scope<NewSecretFormProps>) -> Element {
    cx.render(rsx! {
        {
            LazyNodes::new(|f| f.text(format_args!("<new-secret>")))
        }
        // The form to create an invitation
        form {
            id: "add-secret-form",
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "Add Secret",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
                        Alert {
                            alert_color: AlertColor::Success,
                            class: "mb-3",
                            "All values entered on this form will be End 2 End encrypted before
                            being sent to the server."
                        }
                        Input {
                            id: "secret-name",
                            input_type: InputType::Text,
                            help_text: "The name should be a POSIX compliant environment variable name i.e. upper and lowercase letters, numbers and underscore."
                            required: true,
                            placeholder: "e.g. DATABASE_URL"
                            label: "Name",
                            name: "name"
                        }
                        label {
                            "for": "secret",
                            "Secret"
                        }
                        textarea {
                            rows: "10",
                            id: "secret-value",
                            autocomplete: "off",
                            required: "",
                            name: "secret"
                        }
                        label {
                            "for": "folder",
                            "Environment"
                        }
                        Select {
                            id: "environment_id",
                            name: "environment_id",
                            cx.props.environments.iter().map(|env| {
                                cx.render(rsx! (
                                    option {
                                        value: "{env.id}",
                                        "{env.name}"
                                    }
                                ))
                            })
                        }
                        input {
                            "type": "hidden",
                            id: "encrypted-vault-key",
                            value: "{cx.props.user_vault.encrypted_vault_key}"
                        }
                        input {
                            "type": "hidden",
                            id: "user-vault-ecdh-public-key",
                            value: "{cx.props.user_vault.ecdh_public_key}"
                        }
                        input {
                            "type": "hidden",
                            id: "vault-id",
                            value: "{cx.props.user_vault.vault_id}"
                        }
                        input {
                            "type": "hidden",
                            id: "name-blind-index",
                            name: "name_blind_index"
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        id: "create-secret",
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create"
                    }
                }
            }
        }
        {LazyNodes::new(|f| f.text(format_args!("</new-secret>")))}
    })
}
