#![allow(non_snake_case)]
use db::{Environment, Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps {
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
                            th { "Action" }
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
                                    }
                                    td {
                                    }
                                    td {
                                    }
                                }
                            ))
                        }
                    }
                }
            }
        }
    ))
}
