markup::define! {
    MembersDrawer {

        form.m_form[method = "post", action=super::NEW] {
            members_drawer[label="Add Vault"] {
                template[slot="body"] {
                    p {
                        "Vaults keep related secrets together.
                        For example you could have a vault called My Project with all
                        the secrets related to your project."
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Vault" }
                }
            }
        }

    }
}
