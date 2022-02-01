markup::define! {
    MembersDrawer(vault_name: String) {

        form.m_form[method = "post", action=super::NEW] {
            members_drawer[label=format!("Members of {}", vault_name)] {
                template[slot="body"] {
                    p {
                        "Members of a vault can add and remove secrets. "
                        " They can also create Service Accounts for this Vault"
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Vault" }
                }
            }
        }

    }
}
