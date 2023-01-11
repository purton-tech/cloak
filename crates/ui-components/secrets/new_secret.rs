#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

pub static DRAW_TRIGGER: &str = "add-secret-form";

#[derive(Props, PartialEq, Eq)]
pub struct NewSecretFormProps {
    submit_action: String,
}

pub fn NewSecretForm(cx: Scope<NewSecretFormProps>) -> Element {
    cx.render(rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "{cx.props.submit_action}",
            Drawer {
                label: "Add Secret",
                trigger_id: DRAW_TRIGGER,
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
                        Alert {
                            alert_color: AlertColor::Success,
                            class: "mb-3",
                            "All values entered on this form will be End 2 End encrypted before
                            being sent to the server."
                        }
                        Input {
                            input_type: InputType::Text,
                            help_text: "The name should be a POSIX compliant environment variable name i.e. upper and lowercase letters, numbers and underscore."
                            required: true,
                            placeholder: "e.g. DATABASE_URL"
                            label: "Name",
                            name: "name"
                        }
                        {LazyNodes::new(|f| f.text(format_args!(
                            "<ecdh-keygen public='public_key',
                                private='encrypted_vault_key'></ecdh-keygen>")))}
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Create"
                    }
                }
            }
        }
    })
}
