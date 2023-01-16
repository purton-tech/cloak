#![allow(non_snake_case)]
use db::VaultMember;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps {
    members: Vec<VaultMember>,
    organisation_id: i32,
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
                            th {
                                class: "text-right",
                                "Action" 
                            }
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
                                        class: "text-right",
                                        DropDown {
                                            direction: Direction::SouthWest,
                                            button_text: "...",
                                            DropDownLink {
                                                drawer_trigger: format!("delete-secret-trigger-{}-{}", 
                                                    member.vault_id, member.user_id),
                                                href: "#",
                                                "Remove Member"
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
        cx.props.members.iter().map(|member| {
            cx.render(rsx!(
                super::remove::RemoveMemberDrawer {
                    organisation_id: cx.props.organisation_id,
                    vault_member: member,
                    trigger_id: format!("delete-secret-trigger-{}-{}", 
                        member.vault_id, member.user_id),
                }
            ))
        })
    ))
}
