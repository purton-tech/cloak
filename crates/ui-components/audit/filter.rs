#![allow(non_snake_case)]
use db::Member;
use dioxus::prelude::*;
use primer_rsx::*;

pub static DRAW_TRIGGER: &str = "filter-audit-drawer";

#[derive(Props, PartialEq)]
pub struct FilterDrawerProps {
    team_users: Vec<Member>,
    organisation_id: i32,
    reset_search: bool,
    submit_action: String,
}

pub fn FilterDrawer(cx: Scope<FilterDrawerProps>) -> Element {
    cx.render(rsx! {
        form {
            class: "remember",
            method: "post",
            "data-turbo": "false",
            "data-remember-reset": "{cx.props.reset_search}",
            "data-remember-name": "audit",
            id: "filter-form",
            action: "{cx.props.submit_action}",

            Drawer {
                label: "Filter",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "d-flex flex-column",

                        Select {
                            label: "User",
                            help_text: "For which user do you want to search",
                            name: "user",
                            option {
                                value: "0",
                                "Any"
                            }
                            cx.props.team_users.iter().map(|user| {
                                cx.render(rsx! {
                                    option {
                                        value: "{user.id}",
                                        "{user.email}"
                                    }
                                })
                            })
                        }

                        Select {
                            label: "Access Type",
                            help_text: "Split between user interface and CLI usage.",
                            name: "access_type",
                            option {
                                value: "0",
                                "Any"
                            }
                            option {
                                value: "1",
                                "User Interface"
                            }
                            option {
                                value: "2",
                                "CLI"
                            }
                            option {
                                value: "3",
                                "Service Account"
                            }
                        }

                        Select {
                            label: "Action",
                            help_text: "What action did the user perform",
                            name: "action",
                            option {
                                value: "0",
                                "Any"
                            }
                            option {
                                value: "1",
                                "Add Member"
                            }
                            option {
                                value: "2",
                                "Delete Member"
                            }
                            option {
                                value: "3",
                                "Add Secret"
                            }
                            option {
                                value: "4",
                                "Delete Secret"
                            }
                            option {
                                value: "5",
                                "Access Secret"
                            }
                            option {
                                value: "6",
                                "New Service Account"
                            }
                            option {
                                value: "7",
                                "Delete Service Account"
                            }
                            option {
                                value: "8",
                                "Connect Service Account"
                            }
                            option {
                                value: "9",
                                "Create Invite"
                            }
                            option {
                                value: "10",
                                "Remove Team Member"
                            }
                            option {
                                value: "11",
                                "Create Vault"
                            }
                            option {
                                value: "12",
                                "Delete Vault"
                            }
                        }

                        input {
                            "type": "hidden",
                            name: "id",
                            id: "last-row-id",
                            value: "0"
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Apply Filter"
                    }
                }
            }
        }
    })
}
