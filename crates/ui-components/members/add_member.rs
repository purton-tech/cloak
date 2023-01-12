#![allow(non_snake_case)]
#![allow(unused_braces)]
use db::{Environment, NonMember, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

pub static DRAW_TRIGGER: &str = "add-vault-member-drawer";

#[derive(Props, PartialEq)]
pub struct AddMemberFormProps {
    user_vault: UserVault,
    submit_action: String,
    non_members: Vec<NonMember>,
    environments: Vec<Environment>,
}

pub fn AddMemberForm(cx: Scope<AddMemberFormProps>) -> Element {
    cx.render(rsx! {
        {
            LazyNodes::new(|f| f.text(format_args!("<add-member>")))
        }
        form {
            id: "add-member-form",
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "Add User to Vault",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "d-flex flex-column",

                        Select {
                            id: "user-selection",
                            name: "user_id",
                            label: "User",
                            help_text: "Select a user",
                            cx.props.non_members.iter().map(|user| {
                                cx.render(rsx! (
                                    option {
                                        value: "{user.id}",
                                        "data-ecdh-pub-key": "{user.ecdh_public_key}",
                                        "{user.email}"
                                    }
                                ))
                            })
                        }
                        label {
                            "Which environments do you want the user to have access to?"
                        }
                        cx.props.environments.iter().map(|env| {
                            cx.render(rsx! (
                                label {
                                    "for": "{env.name}",
                                    input {
                                        "type": "checkbox",
                                        name: "env",
                                        id: "{env.name}",
                                        value: "{env.id}",
                                        "{env.name}"
                                    }
                                }
                            ))
                        })
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
                        //  These hidden fields are populated by the add_member.ts
                        input {
                            "type": "hidden",
                            id: "environments",
                            name: "environments"
                        }
                        input {
                            "type": "hidden",
                            id: "wrapped-vault-key",
                            name: "wrapped_vault_key"
                        }
                        input {
                            "type": "hidden",
                            id: "ecdh-public-key",
                            name: "ecdh_public_key"
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        id: "add-member-button",
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Add User to Vault"
                    }
                }
            }
        }
        {LazyNodes::new(|f| f.text(format_args!("</add-member>")))}
    })
}
