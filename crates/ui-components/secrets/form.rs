#![allow(non_snake_case)]
use db::{Environment, Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct SecretFormProps<'a> {
    submit_action: String,
    user_vault: &'a UserVault,
    environments: &'a Vec<Environment>,
    // If the secret is included then edit the existing secret
    secret: Option<&'a Secret>,
    trigger_id: String,
}

/***
 * This form is enhanced by asset-pipeline/web-components/new-secret.ts
 */
pub fn SecretForm<'a>(cx: Scope<'a, SecretFormProps<'a>>) -> Element {
    let name = if let Some(secret) = &cx.props.secret {
        &secret.name
    } else {
        ""
    };

    let value = if let Some(secret) = &cx.props.secret {
        &secret.secret
    } else {
        ""
    };

    cx.render(rsx! {
        form {
            id: "add-secret-form",
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "Add Secret",
                trigger_id: &cx.props.trigger_id,
                component_name: "new-secret",
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
                            value: name,
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
                            name: "secret",
                            "{value}"
                        }
                        label {
                            "for": "folder",
                            "Environment"
                        }
                        Select {
                            id: "environment_id",
                            name: "environment_id",
                            required: true,
                            option {
                                "Please Select..."
                            }
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
    })
}
