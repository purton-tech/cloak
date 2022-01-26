markup::define! {
    InviteUserPage {

        form.m_form {
            invite_user[label="Invite User"] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }

                    fieldset {
                        label[for="email"] { "Email" }
                        input[type="email", required="", name="name"] {}
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
