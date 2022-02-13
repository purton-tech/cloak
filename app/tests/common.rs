use rand::Rng;
use sqlx::PgPool;
use std::env;
use thirtyfour::prelude::*; // Or `Aes128Gcm`

#[derive(Clone, Debug)]
pub struct Config {
    pub webdriver_url: String,
    pub host: String,
    // The database
    pub db_pool: PgPool,
    pub headless: bool,
}

impl Config {
    pub async fn new() -> Config {
        let webdriver_url: String = if env::var("WEB_DRIVER_URL").is_ok() {
            env::var("WEB_DRIVER_URL").unwrap()
        } else {
            // Default to selenium in our dev container
            "http://selenium:4444".into()
        };

        let headless = env::var("ENABLE_HEADLESS").is_ok();

        let host = if env::var("WEB_DRIVER_DESTINATION_HOST").is_ok() {
            env::var("WEB_DRIVER_DESTINATION_HOST").unwrap()
        } else {
            "http://envoy:7100".into()
        };

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPool::connect(&database_url).await.unwrap();

        Config {
            webdriver_url,
            host,
            db_pool,
            headless,
        }
    }

    pub async fn get_driver(&self) -> WebDriverResult<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-gpu")?;
        caps.add_chrome_arg("--start-maximized")?;
        // We need the below otherwise window.crypto.subtle is not defined
        caps.add_chrome_arg("--unsafely-treat-insecure-origin-as-secure=http://envoy:7100")?;

        if self.headless {
            caps.set_headless()?;
        }
        WebDriver::new(&self.webdriver_url, &caps).await
    }
}

pub async fn register_random_user(driver: &WebDriver) -> WebDriverResult<String> {
    let email = random_email();

    // Register someone
    driver
        .find_element(By::LinkText("SIGN UP"))
        .await?
        .click()
        .await?;
    driver
        .find_element(By::Id("email"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find_element(By::Id("password"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find_element(By::Id("confirm_password"))
        .await?
        .send_keys(&email)
        .await?;
    driver
        .find_element(By::Css("button[type='submit']"))
        .await?
        .click()
        .await?;

    // OTP Code
    // Wait for page to load as code might not be in database yet.
    driver.find_element(By::Id("code")).await?;

    Ok(email)
}

pub async fn force_otp(config: &Config) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE sessions SET otp_code_confirmed = true",)
        .execute(&config.db_pool)
        .await?;

    Ok(())
}

pub async fn count_service_account_secrets(
    config: &Config,
    email: &str,
) -> Result<i64, sqlx::Error> {
    let vault_id = sqlx::query!(
        "
            SELECT 
                id
            FROM 
                vaults 
            WHERE
                user_id IN
                    (SELECT id FROM users WHERE email=$1)
        ",
        email
    )
    .fetch_one(&config.db_pool)
    .await?;

    dbg!(&vault_id);

    let count = sqlx::query!(
        "
            SELECT 
                count(*)
            FROM 
                service_account_secrets 
            WHERE
                service_account_id IN (SELECT id FROM service_accounts WHERE vault_id = $1)
        ",
        vault_id.id
    )
    .fetch_one(&config.db_pool)
    .await?;

    if let Some(count) = count.count {
        return Ok(count);
    }

    Err(sqlx::Error::Protocol("Failed".to_string()))
}

pub async fn count_secrets(config: &Config, email: &str) -> Result<i64, sqlx::Error> {
    let user = sqlx::query!(
        "
            SELECT 
                id
            FROM 
                users 
            WHERE
                email = $1
        ",
        email
    )
    .fetch_one(&config.db_pool)
    .await?;

    let count = sqlx::query!(
        "
            SELECT 
                count(*)
            FROM 
                secrets 
            WHERE
                vault_id IN (SELECT vault_id FROM users_vaults WHERE USER_ID = $1)
        ",
        user.id
    )
    .fetch_one(&config.db_pool)
    .await?;

    if let Some(count) = count.count {
        return Ok(count);
    }

    Err(sqlx::Error::Protocol("Failed".to_string()))
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

pub fn random_email() -> String {
    let mut rng = rand::thread_rng();
    format!("{}@test.com", rng.gen::<u32>())
}
