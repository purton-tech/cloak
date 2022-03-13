use lettre::message;
use std::env;

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
        if let Ok(host) = env::var("SMTP_HOST") {
            if let Ok(username) = env::var("SMTP_USERNAME") {
                if let Ok(password) = env::var("SMTP_PASSWORD") {
                    if let Ok(smtp_port) = env::var("SMTP_PORT") {
                        if let Ok(domain) = env::var("INVITE_DOMAIN") {
                            if let Ok(from_email) = env::var("INVITE_FROM_EMAIL_ADDRESS") {
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
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
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
}
