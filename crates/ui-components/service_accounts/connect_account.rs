#![allow(non_snake_case)]
use db::{EnvironmentsAndVault, ServiceAccount};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct FormProps<'a> {
    submit_action: String,
    drawer_trigger: &'a str,
    service_account: &'a ServiceAccount,
    environments_and_vaults: &'a Vec<EnvironmentsAndVault>,
    team_id: i32,
}

pub fn ConnectAccountForm<'a>(cx: Scope<'a, FormProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Connect to Vault",
            trigger_id: "{cx.props.drawer_trigger}",
            component_name: "connect-account",
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Select {
                        id: "vault-select-{cx.props.service_account.id}",
                        name: "environment_id",
                        label: "Which Vault would you like to connect to?",
                        option {
                            "Select..."
                        }
                        cx.props.environments_and_vaults.iter().map(|env| {
                            cx.render(rsx! (
                                option {
                                    value: "{env.vault_id}:{env.id}",
                                    "Vault: {env.vault_name}, Environment: {env.name}"
                                }
                            ))
                        })
                    }
                }

                input {
                    id: "service-account-id",
                    "type": "hidden",
                    value: "{cx.props.service_account.id}",
                    name: "service_account_id"
                }

                input {
                    id: "service-account-public-key-{cx.props.service_account.id}",
                    "type": "hidden",
                    value: "{cx.props.service_account.ecdh_public_key}",
                    name: "public_key"
                }

                form {
                    method: "post",
                    action: "{cx.props.submit_action}",
                    id: "service-account-form-{cx.props.service_account.id}",
                    input {
                        "type": "hidden",
                        name: "service_account_id",
                        value: "{cx.props.service_account.id}"
                    }
                    input {
                        id: "service-account-form-environment-id-{cx.props.service_account.id}",
                        "type": "hidden",
                        name: "environment_id",
                        value: "{cx.props.service_account.id}"
                    }
                    input {
                        id: "service-account-form-vault-id-{cx.props.service_account.id}",
                        "type": "hidden",
                        name: "vault_id",
                        value: "{cx.props.service_account.id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    id: "connect-to-vault-{cx.props.service_account.id}",
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Connect to Vault"
                }
            }
        }
    })
}
