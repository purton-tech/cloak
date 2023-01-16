#![allow(non_snake_case)]
use db::{Environment, Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps {
    organisation_id: i32,
    environment: Environment,
    secrets: Vec<Secret>,
    user_vault: UserVault,
}

pub fn SecretsTable(cx: Scope<TableProps>) -> Element {
    cx.render(rsx!(
        Box {
            BoxHeader {
                title: "Secrets"
            }
            BoxBody {
                DataTable {
                    table {
                        thead {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                            th {
                                class: "text-right",
                                "Action" 
                            }
                        }
                        tbody {
                            cx.props.secrets.iter().map(|secret| rsx!(
                                tr {
                                    td {
                                        {LazyNodes::new(|f| f.text(format_args!(
                                            "<ecdh-cipher cipher='{}'
                                            wrapped-aes-key='{}' 
                                            ecdh-public-key='{}'></ecdh-cipher>",
                                            secret.name,
                                            cx.props.user_vault.encrypted_vault_key,
                                            cx.props.user_vault.ecdh_public_key
                                        )))}
                                    }
                                    td {
                                        RelativeTime {
                                            format: RelativeTimeFormat::Datetime,
                                            datetime: &secret.updated_at
                                        }
                                    }
                                    td {
                                        RelativeTime {
                                            format: RelativeTimeFormat::Datetime,
                                            datetime: &secret.created_at
                                        }
                                    }
                                    td {
                                        class: "text-right",
                                        DropDown {
                                            direction: Direction::SouthWest,
                                            button_text: "...",
                                            DropDownLink {
                                                drawer_trigger: format!("delete-secret-trigger-{}", secret.id),
                                                href: "#",
                                                "Delete Secret"
                                            }
                                        }
                                    }
                                }
                            ))
                        }
                    }
                }
            }
        }
        // Create all the delete drawers
        cx.props.secrets.iter().map(|secret| {
            cx.render(rsx!(
                super::delete::DeleteSecretDrawer {
                    organisation_id: cx.props.organisation_id,
                    user_vault: &cx.props.user_vault,
                    secret: secret,
                    trigger_id: format!("delete-secret-trigger-{}", secret.id),
                }
            ))
        })
    ))
}
