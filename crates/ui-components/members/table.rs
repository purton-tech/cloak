#![allow(non_snake_case)]
use db::VaultMember;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps {
    members: Vec<VaultMember>,
}

pub fn MembersTable(cx: Scope<TableProps>) -> Element {
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
                            th { "Environments" }
                            th { "Action" }
                        }
                        tbody {
                            cx.props.members.iter().map(|member| rsx!(
                                tr {
                                    td {
                                        "{member.email}"
                                    }
                                    td {
                                        if let Some(env) = &member.environments {
                                            cx.render(rsx!(
                                                "{env}"
                                            ))
                                        } else {
                                            None
                                        }
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
