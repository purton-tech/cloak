#![allow(non_snake_case)]
use db::ServiceAccount;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct DrawerProps<'a> {
    organisation_id: i32,
    service_account: &'a ServiceAccount,
    trigger_id: String,
}

pub fn DeleteServiceAccoutDrawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::service_accounts::delete_route(cx.props.organisation_id),
            label: "Delete Service Account ?",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to delete the service account
                            '{cx.props.service_account.account_name}'"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "service_account_id",
                        "value": "{cx.props.service_account.id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Delete Service Account"
                }
            }
        }
    })
}
