#![allow(non_snake_case)]
use db::ServiceAccount;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct ViewAccountDrawerProps<'a> {
    drawer_trigger: &'a str,
    service_account: &'a ServiceAccount,
}

pub fn ViewAccountDrawer<'a>(cx: Scope<'a, ViewAccountDrawerProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Integrations",
            trigger_id: "{cx.props.drawer_trigger}",
            DrawerBody {
                div {
                    class: "d-flex flex-column",

                    TabContainer {
                        tabs: cx.render(rsx! {
                            TabHeader {
                                selected: true,
                                tab: "Local",
                                name: "Local"
                            }
                        })
                        TabPanel {
                            hidden: true,
                            id: "Local",
                            h1 {
                                "Hello"
                            }
                        }
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_scheme: ButtonScheme::Danger,
                    "Close"
                }
            }
        }
    })
}
