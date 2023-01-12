pub mod audit;
pub mod cloak_layout;
pub mod logout_form;
pub mod members;
pub mod profile;
pub mod profile_popup;
pub mod secrets;
pub mod service_accounts;
pub mod team_members;
pub mod teams;
pub mod vaults;

pub mod routes {

    pub mod members {
        pub fn member_route(vault_id: i32, organisation_id: i32) -> String {
            format!("/app/team/{}/vault/{}/members", organisation_id, vault_id)
        }

        pub fn add_route(vault_id: i32, organisation_id: i32) -> String {
            format!(
                "/app/team/{}/vault/{}/members/add",
                organisation_id, vault_id
            )
        }

        pub fn delete_route(vault_id: i32, organisation_id: i32) -> String {
            format!(
                "/app/team/{}/vault/{}/members/delete",
                organisation_id, vault_id
            )
        }
    }

    pub mod secrets {
        pub fn index_route(vault_id: i32, organisation_id: i32) -> String {
            format!("/app/team/{}/vault/{}/secrets", organisation_id, vault_id)
        }

        pub fn new_route(vault_id: i32, organisation_id: i32) -> String {
            format!(
                "/app/team/{}/vault/{}/secrets/new",
                organisation_id, vault_id
            )
        }

        pub fn delete_route(vault_id: i32, organisation_id: i32) -> String {
            format!(
                "/app/team/{}/vault/{}/secrets/delete",
                organisation_id, vault_id
            )
        }
    }

    pub mod vaults {
        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/vaults", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/new_vault", organisation_id)
        }

        pub fn delete_route(organisation_id: i32) -> String {
            format!("/app/team/{}/vaults/delete", organisation_id)
        }
    }

    pub mod audit {
        pub static INDEX: &str = "/app/team/:team_id/audit";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/audit", organisation_id)
        }
    }

    pub mod service_accounts {
        pub static INDEX: &str = "/app/team/:team_id/api_keys";
        pub static NEW: &str = "/app/team/:team_id/api_keys/new";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/service_accounts", organisation_id)
        }

        pub fn delete_route(organisation_id: i32) -> String {
            format!("/app/team/{}/service_accounts/delete", organisation_id)
        }

        pub fn connect_route(organisation_id: i32) -> String {
            format!("/app/team/{}/service_accounts/connect", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/service_accounts/new", organisation_id)
        }
    }

    pub mod team {
        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}", organisation_id)
        }

        pub fn switch_route(organisation_id: i32) -> String {
            format!("/app/team/{}/switch", organisation_id)
        }

        pub fn teams_popup_route(organisation_id: i32) -> String {
            format!("/app/team/{}/teams_popup", organisation_id)
        }

        pub fn create_route(organisation_id: i32) -> String {
            format!("/app/team/{}/create_invite", organisation_id)
        }

        pub fn delete_route(organisation_id: i32) -> String {
            format!("/app/team/{}/delete", organisation_id)
        }

        pub fn set_name_route(organisation_id: i32) -> String {
            format!("/app/team/{}/set_name", organisation_id)
        }

        pub fn new_team_route(organisation_id: i32) -> String {
            format!("/app/team/{}/new", organisation_id)
        }
    }

    pub mod profile {

        pub fn set_details_route(organisation_id: i32) -> String {
            format!("/app/team/{}/set_details", organisation_id)
        }

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/profile", organisation_id)
        }

        pub fn profile_popup_route(organisation_id: i32) -> String {
            format!("/app/team/{}/profile_popup", organisation_id)
        }
    }

    pub mod api_keys {
        pub static INDEX: &str = "/app/team/:team_id/api_keys";
        pub static NEW: &str = "/app/team/:team_id/api_keys/new";

        pub fn index_route(organisation_id: i32) -> String {
            format!("/app/team/{}/api_keys", organisation_id)
        }

        pub fn new_route(organisation_id: i32) -> String {
            format!("/app/team/{}/api_keys/new", organisation_id)
        }
    }
}
