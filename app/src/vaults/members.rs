use crate::models;

markup::define! {
    MembersDrawer<'a>(vault_name: String, members: &'a Vec<models::user_vault::UserDetails>) {

        form.m_form[method = "post", action=super::NEW] {
            members_drawer[label=format!("Members of {}", vault_name)] {

                template[slot="body"] {
                    p {
                        "Members of a vault can add and remove secrets. "
                        " They can also create Service Accounts for this Vault"
                    }

                    ul {
                        @for member in *members {
                            li {
                                {member.email}
                            }
                        }
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success[type = "submit"] { "Create Vault" }
                }
            }
        }

    }
}
