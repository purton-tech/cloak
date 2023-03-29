#![allow(non_snake_case)]
use db::{Secret, UserVault};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct SecretViewProps<'a> {
    secret: &'a Secret,
    user_vault: &'a UserVault,
    trigger_id: String,
}

pub fn SecretView<'a>(cx: Scope<'a, SecretViewProps<'a>>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "View Secret",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",

                    h4 {
                        {LazyNodes::new(|f| f.text(format_args!(
                            "<ecdh-cipher cipher='{}'
                            wrapped-aes-key='{}' 
                            ecdh-public-key='{}'></ecdh-cipher>",
                            cx.props.secret.name,
                            cx.props.user_vault.encrypted_vault_key,
                            cx.props.user_vault.ecdh_public_key
                        )))}
                    }

                    {LazyNodes::new(|f| f.text(format_args!(
                        "<ecdh-cipher class='border mt-3 p-1' cipher='{}'
                        wrapped-aes-key='{}' 
                        ecdh-public-key='{}'></ecdh-cipher>",
                        cx.props.secret.secret,
                        cx.props.user_vault.encrypted_vault_key,
                        cx.props.user_vault.ecdh_public_key
                    )))}
                }
            }
            DrawerFooter {

            }
        }
    })
}
