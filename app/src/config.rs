use lettre::message;
use rustls::ClientConfig;
use std::env;
use tokio_postgres::NoTls;
use tokio_postgres_rustls::MakeRustlsConnect;

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    // Configure SMTP for email.
    pub host: String,
    pub port: u16,
    pub tls_off: bool,
    pub username: String,
    pub password: String,
    pub domain: String,
    pub from_email: message::Mailbox,
}

impl SmtpConfig {
    pub fn new() -> Option<SmtpConfig> {
        let host = env::var("SMTP_HOST");
        let username = env::var("SMTP_USERNAME");
        let password = env::var("SMTP_PASSWORD");
        let smtp_port = env::var("SMTP_PORT");
        let domain = env::var("INVITE_DOMAIN");
        let from_email = env::var("INVITE_FROM_EMAIL_ADDRESS");

        if let (Ok(host), Ok(username), Ok(password), Ok(smtp_port), Ok(domain), Ok(from_email)) =
            (host, username, password, smtp_port, domain, from_email)
        {
            Some(SmtpConfig {
                host,
                port: smtp_port.parse::<u16>().unwrap(),
                tls_off: env::var("SMTP_TLS_OFF").is_ok(),
                username,
                password,
                domain,
                from_email: from_email.parse().unwrap(),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    // The gRPC server
    pub app_database_url: String,
    // Configure SMTP for email.
    pub smtp_config: Option<SmtpConfig>,
}

impl Config {
    pub fn new() -> Config {
        let port: u16 = if env::var("PORT").is_ok() {
            env::var("PORT").unwrap().parse::<u16>().unwrap()
        } else {
            7103
        };

        let app_database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        Config {
            port,
            app_database_url,
            smtp_config: SmtpConfig::new(),
        }
    }

    pub fn create_pool(&self) -> deadpool_postgres::Pool {
        // Example to parse
        // APP_DATABASE_URL=postgresql://cloak:testpassword@db:5432/cloak?sslmode=disable
        let mut cfg = deadpool_postgres::Config::new();
        let url: Vec<&str> = if self.app_database_url.starts_with("postgresql://") {
            self.app_database_url.split("postgresql://").collect()
        } else {
            self.app_database_url.split("postgres://").collect()
        };
        let split_on_at: Vec<&str> = url[1].split('@').collect();
        let user_and_pass: Vec<&str> = split_on_at[0].split(':').collect();

        let split_on_slash: Vec<&str> = split_on_at[1].split('/').collect();
        let host_and_port: Vec<&str> = split_on_slash[0].split(':').collect();
        let dbname_and_params: Vec<&str> = split_on_slash[1].split('?').collect();

        // we need to repalce %40 with @ so this works on Azure Postgres
        cfg.user = Some(user_and_pass[0].replace("%40", "@"));
        cfg.password = Some(String::from(user_and_pass[1]));
        cfg.host = Some(String::from(host_and_port[0]));
        cfg.port = Some(host_and_port[1].parse::<u16>().unwrap());
        cfg.dbname = Some(String::from(dbname_and_params[0]));

        if self.app_database_url.contains("sslmode=require") {
            let mut root_store = rustls::RootCertStore::empty();
            root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
                |ta| {
                    rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                },
            ));

            let tls_config = ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_store)
                .with_no_client_auth();
            let tls = MakeRustlsConnect::new(tls_config);
            cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), tls)
                .unwrap()
        } else {
            cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
                .unwrap()
        }
    }
}
