markup::define! {
    AddMemberDrawer {

        form.m_form {
            add_member[label="Add Member"] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success { "Create Invitation" }
                    button.a_button.auto.danger { "Cancel" }
                }
            }
        }
    }
}
