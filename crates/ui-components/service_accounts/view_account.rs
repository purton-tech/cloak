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
                    class: "d-flex flex-column p-2",

                    TabContainer {
                        tabs: cx.render(rsx! {
                            TabHeader {
                                selected: true,
                                tab: "Local",
                                name: "Local"
                            }
                            TabHeader {
                                selected: false,
                                tab: "Kubernetes",
                                name: "Kubernetes"
                            }
                        })
                        TabPanel {
                            hidden: false,
                            id: "Local",
                            h5 {
                                "1. Install the cloak CLI"
                            }
                            p {
                                class: "mb-2",
                                a {
                                    href: "https://cloak.software/docs/getting-started/introduction/",
                                    target: "_blank",
                                    "Getting Started Docs"
                                }
                            }
                            h5 {
                                "2. Download the private key for this service account"
                            }
                            p {
                                class: "mb-2",
                                "Click here"
                                //<a id='@format!("wrapped-ecdh-private-key-{}", service_account.id)' 
                                //    data-key="@service_account.encrypted_ecdh_private_key"
                                //    download="cloak.pem">
                                //    cloak.pem
                                //</a>
                                "to download your private key. 
                                Store it in the folder where you will use the cloak CLI tool."
                            }
                            h5 {
                                "3. View the secrets"
                            }
                            p {
                                class: "mb-2",
                                "From the same folder where you downloaded your <code>cloak.pem</code> file run..."
                                code {
                                    "$ cloak secrets"
                                }
                            }
                            h5 {
                                "4. Store secrets in a .env file (Optional)"
                            }
                            p {
                                class: "mb-2",
                                "From the same folder where you downloaded your <code>cloak.pem</code> file run..."
                                code {
                                    "$ cloak env > .env"
                                }
                            }
                            h5 {
                                "5. Inject secrets into as env vars into a process (Optional)"
                            }
                            p {
                                class: "mb-2",
                                "From the same folder where you downloaded your <code>cloak.pem</code> file run..."
                                code {
                                    "$ cloak run name-of-process"
                                }
                            }
                        }
                        TabPanel {
                            hidden: true,
                            id: "Kubernetes",
                            h5 {
                                "1. Install the cloak CLI"
                            }
                            p {
                                class: "mb-2",
                                a {
                                    href: "https://cloak.software/docs/getting-started/introduction/",
                                    target: "_blank",
                                    "Getting Started Docs"
                                }
                            }
                            h5 {
                                "2. Download the private key for this service account"
                            }
                            p {
                                class: "mb-2",
                                "Click here"
                                //<a id='@format!("wrapped-ecdh-private-key-{}", service_account.id)' 
                                //    data-key="@service_account.encrypted_ecdh_private_key"
                                //    download="cloak.pem">
                                //    cloak.pem
                                //</a>
                                "to download your private key. 
                                Store it in the folder where you will use the cloak CLI tool."
                            }
                            h5 {
                                "3. View the secrets"
                            }
                            p {
                                class: "mb-2",
                                "From the same folder where you downloaded your <code>cloak.pem</code> file run..."
                                code {
                                    "$ cloak secrets"
                                }
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
