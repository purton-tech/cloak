#![allow(non_snake_case)]
use super::index::VaultSummary;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct TableProps<'a> {
    vaults: &'a Vec<VaultSummary>,
    organisation_id: i32,
}

pub fn VaultTable<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    cx.render(rsx!(
        Box {
            class: "has-data-table",
            BoxHeader {
                title: "Your Vaults"
            }
            BoxBody {
                DataTable {
                    table {
                        thead {
                            tr {
                                th { "Name" }
                                th { "Created" }
                                th { "Members" }
                                th { "Secrets" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                        }
                        tbody {
                            cx.props.vaults.iter().map(|vault| rsx!(
                                tr {
                                    td {
                                        strong {
                                            a {
                                                href: "{vault.href}",
                                                "{vault.name}"
                                            }
                                        }
                                    }
                                    td {
                                        RelativeTime {
                                            format: RelativeTimeFormat::Datetime,
                                            datetime: &vault.created_at
                                        }
                                    }
                                    td {
                                        Label {
                                            "{vault.user_count}"
                                        }
                                    }
                                    td {
                                        Label {
                                            label_color: LabelColor::Attention,
                                            "{vault.secrets_count}"
                                        }
                                    }
                                    td {
                                        class: "text-right",
                                        DropDown {
                                            direction: Direction::SouthWest,
                                            button_text: "...",
                                            DropDownLink {
                                                drawer_trigger: format!("delete-vault-trigger-{}", vault.id),
                                                href: "#",
                                                target: "_top",
                                                "Delete Vault"
                                            }
                                        }
                                    }
                                }
                            ))
                        }
                    }
                }
                // Create all the delete drawers
                cx.props.vaults.iter().map(|vault| {
                    cx.render(rsx!(
                        super::delete::DeleteVaultDrawer {
                            organisation_id: cx.props.organisation_id,
                            vault: vault,
                            trigger_id: format!("delete-vault-trigger-{}", vault.id),
                        }
                    ))
                })
            }
        }
    ))
}
