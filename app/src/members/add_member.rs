use crate::models;

markup::define! {
    AddMemberDrawer<'a>(team: &'a Vec<models::organisation::User>) {

        form.m_form {
            add_member[label="Add Member"] {
                template[slot="body"] {
                    p {
                        "Invite people into your team."
                    }

                    table.m_table {
                        thead {
                            tr {
                                th { "Email" }
                                th { "Add" }
                            }
                        }
                        tbody {
                            @for user in *team {
                                tr {
                                    td {
                                        {user.email}
                                    }
                                    td {
                                        input[type="checkbox", value=format!("{}", user.id)] {}
                                    }
                                }
                            }
                        }
                    }
                }

                template[slot="footer"] {
                    button.a_button.auto.success { "Update Members" }
                    button.a_button.auto.danger { "Cancel" }
                }
            }
        }
    }
}
