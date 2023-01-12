#![allow(non_snake_case)]
use super::index::VaultSummary;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct TableProps<'a> {
    vaults: &'a Vec<VaultSummary>,
}

pub fn VaultTable<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    cx.render(rsx!(
        Box {
            BoxHeader {
                title: "Your Vaults"
            }
            BoxBody {
                DataTable {
                    table {
                        thead {
                            th { "Name" }
                            th { "Created" }
                            th { "Members" }
                            th { "Secrets" }
                        }
                        tbody {
                            cx.props.vaults.iter().map(|vault| rsx!(
                                tr {
                                    td {
                                        a {
                                            href: "{vault.href}",
                                            "{vault.name}"
                                        }
                                    }
                                    td {
                                        "{vault.name}"
                                    }
                                    td {
                                        "{vault.user_count}"
                                    }
                                    td {
                                        "{vault.secrets_count}"
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
