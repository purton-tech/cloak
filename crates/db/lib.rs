use std::str::FromStr;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use tokio_postgres::Error as TokioPostgresError;

pub use queries::audit::Audit;
pub use queries::environments::{Environment, EnvironmentsAndVault};
pub use queries::invitations::Invitation;
pub use queries::organisations::GetTeams as Team;
pub use queries::organisations::GetUsers as Member;
pub use queries::organisations::Organisation;
pub use queries::secrets::Secret;
pub use queries::service_accounts::ServiceAccount;
pub use queries::user_vaults::{NonMember, UserVault, VaultMember};
pub use queries::users::User;
pub use types::public::Role;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();

    let manager = if database_url.contains("sslmode=require") {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

        let tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
        deadpool_postgres::Manager::new(config, tls)
    } else {
        deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
    };

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
