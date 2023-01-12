#![allow(non_snake_case)]
use db::{EnvironmentsAndVault, ServiceAccount};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps {
    service_accounts: Vec<ServiceAccount>,
    environments_and_vaults: Vec<EnvironmentsAndVault>,
    team_id: i32,
}

pub fn ServiceAccountTable(cx: Scope<TableProps>) -> Element {
    cx.render(rsx!(
        Box {
            BoxHeader {
                title: "Service Accounts"
            }
            BoxBody {
                DataTable {
                    table {
                        thead {
                            th { "Service Account Name" }
                            th { "Vault" }
                            th { "Environment" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Action" }
                        }
                        tbody {
                            cx.props.service_accounts.iter().map(|service_account| rsx!(
                                tr {
                                    if let Some(vault_name) = &service_account.vault_name {
                                        cx.render(rsx!(
                                            td {
                                                id: "service-account-view-{service_account.id}",
                                                "{service_account.account_name}"
                                            }
                                            td {
                                                "{vault_name}"
                                            }
                                        ))
                                    } else {
                                        cx.render(rsx!(
                                            td {
                                                "{service_account.account_name}"
                                            }
                                            td {
                                                a {
                                                    href: "#",
                                                    "data-drawer-target": "service-account-row-{service_account.id}",
                                                    "Connect to Vault"
                                                }
                                                super::connect_account::ConnectAccountForm {
                                                    drawer_trigger: "service-account-row-{service_account.id}",
                                                    submit_action: crate::routes::service_accounts::connect_route(cx.props.team_id),
                                                    service_account: service_account,
                                                    environments_and_vaults: &cx.props.environments_and_vaults,
                                                    team_id: cx.props.team_id
                                                }
                                            }
                                        ))
                                    }
                                    if let Some(env_name) = &service_account.environment_name {
                                        cx.render(rsx!(
                                            td {
                                                Label {
                                                    "{env_name}"
                                                }
                                            }
                                        ))
                                    } else {
                                        cx.render(rsx!(
                                            td {
                                            }
                                        ))
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
