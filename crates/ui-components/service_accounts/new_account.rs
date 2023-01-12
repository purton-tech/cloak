#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

pub static DRAW_TRIGGER: &str = "new-account-drawer";

#[derive(Props, PartialEq, Eq)]
pub struct FormProps {
    submit_action: String,
}

pub fn NewAccountForm(cx: Scope<FormProps>) -> Element {
    cx.render(rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "Add Service Account",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
                        Alert {
                            alert_color: AlertColor::Success,
                            class: "mb-3",
                            "To allow applications to access secrets without human intervention,
                            We support service accounts. A service account is a non-human account 
                            that is tied to one or more vaults."
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "Give your service account a name"
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        {LazyNodes::new(|f| f.text(format_args!(
                            "<ecdh-keygen public='public_key',
                                private='encrypted_private_key'></ecdh-keygen>")))}
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create Service Account"
                    }
                }
            }
        }
    })
}
