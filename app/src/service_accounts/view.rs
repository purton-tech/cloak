markup::define! {
    ViewServiceAccount<'a>(
        service_account: &'a crate::models::service_account::ServiceAccount) {

        form.m_form {
            view_account[label=format!("View {}", service_account.name),
            "service-account-id"=format!("{}", service_account.id)] {

                template[slot="body"] {

                    fieldset {
                        label[for="public_key"] { "ECDH Public Key" }
                        textarea[
                            id=format!("ecdh-public-key-{}", service_account.id),
                            rows="6", type="text",
                            name="public_key"] {
                                {service_account.ecdh_public_key.clone()}
                            }

                        label[for="encrypted_private_key"] { "ECDH Private Key" }
                        textarea[rows="8", required="", readonly="",
                            name="encrypted_private_key",
                            id=format!("wrapped-ecdh-private-key-{}", service_account.id)] {
                            {service_account.encrypted_ecdh_private_key}
                        }
                        span.a_help_text { "The key for this service account" }
                    }
                }

                template[slot="footer"] {}
            }
        }
    }
}
