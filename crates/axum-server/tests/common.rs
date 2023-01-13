use db::Pool;
use rand::Rng;
use std::env;
use thirtyfour::{components::select::SelectElement, prelude::*};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct Config {
    pub webdriver_url: String,
    pub host: String,
    // The database
    pub db_pool: Pool,
    pub headless: bool,
    pub mailhog_url: String,
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

        let mailhog_url = if env::var("MAILHOG_URL").is_ok() {
            env::var("MAILHOG_URL").unwrap()
        } else {
            "http://smtp:8025/api/v2/messages?limit=1".into()
        };

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let db_pool = db::create_pool(&database_url);

        Config {
            webdriver_url,
            host,
            db_pool,
            headless,
            mailhog_url,
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

pub async fn add_secrets(
    driver: &WebDriver,
    name: &str,
    value: &str,
    selector: &str,
) -> WebDriverResult<()> {
    let new_secret_button = driver
        .find_element(By::XPath(
            "//turbo-frame//button[text()='Create A New Secret']",
        ))
        .await?;
    new_secret_button.click().await?;

    let name_field = driver.find_element(By::Css("input[name='name']")).await?;
    name_field.send_keys(name).await?;

    let secret_field = driver
        .find_element(By::Css("textarea[name='secret']"))
        .await?;
    secret_field.send_keys(value).await?;

    let submit_button = driver
        .find_element(By::XPath("//footer//button[text()='Create']"))
        .await?;
    submit_button.click().await?;

    // Make sure the drawer is gone
    let pause = std::time::Duration::from_millis(500);
    std::thread::sleep(pause);

    let ecdh_cipher = driver.find_element(By::Css(selector)).await?;
    assert_eq!(ecdh_cipher.text().await?, name);

    Ok(())
}

pub async fn create_a_vault(driver: &WebDriver) -> WebDriverResult<()> {
    let new_vault_button = driver
        .find_element(By::XPath(
            "//turbo-frame//button[text()='Create A New Vault']",
        ))
        .await?;
    new_vault_button.click().await?;

    let name_field = driver.find_element(By::Css("input[name='name']")).await?;
    name_field.send_keys("My Vault").await?;

    let submit_button = driver
        .find_element(By::XPath("//turbo-frame//footer//button[text()='Create']"))
        .await?;
    submit_button.click().await?;

    let vault_card = driver.find_element(By::LinkText("My Vault")).await?;
    vault_card.click().await?;

    Ok(())
}

pub async fn register_user(driver: &WebDriver, config: &Config) -> WebDriverResult<String> {
    driver.get(format!("{}/auth/sign_up", &config.host)).await?;

    let email = register_random_user(driver).await?;

    force_otp(config).await;

    driver.get(format!("{}/auth/decrypt", config.host)).await?;

    Ok(email)
}

pub async fn select_first_vault(driver: &WebDriver) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::LinkText("Vaults"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::XPath("//tbody/tr[last()]/td[1]/a"))
        .await?
        .click()
        .await?;

    Ok(())
}

pub async fn register_random_user(driver: &WebDriver) -> WebDriverResult<String> {
    let email = random_email();

    // Register someone
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

pub async fn force_otp(config: &Config) {
    let client = config.db_pool.get().await.unwrap();
    let stmt = client
        .prepare_cached("UPDATE sessions SET otp_code_confirmed = true")
        .await
        .unwrap();
    client.execute(&stmt, &[]).await.unwrap();
}

pub async fn add_service_account(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .find_element(By::LinkText("Service Accounts"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::XPath("//button[text()='Add Service Account']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::Css("input[name='name']"))
        .await?
        .send_keys("My Dev Machine")
        .await?;

    driver
        .find_element(By::XPath("//button[text()='Create Service Account']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::LinkText("Connect to Vault"))
        .await?
        .click()
        .await?;

    let vault_selector = driver.find_element(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&vault_selector).await?;
    select
        .select_by_exact_text("Vault: My Vault, Environment: Development")
        .await?;

    driver
        .find_element(By::XPath("//button[text()='Connect to Vault']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::LinkText("My Dev Machine"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::LinkText("X"))
        .await?
        .click()
        .await?;

    Ok(())
}

pub async fn count_service_account_secrets(config: &Config, email: &str) -> i64 {
    let client = config.db_pool.get().await.unwrap();
    let stmt = client
        .prepare_cached(
            "
        SELECT 
            id
        FROM 
            vaults 
        WHERE
            id IN
                (SELECT vault_id FROM users_vaults uv 
                WHERE uv.user_id IN (SELECT id FROM users WHERE email=$1))
    ",
        )
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&email]).await.unwrap();
    let vault_id: i32 = rows[0].get(0);

    let stmt = client
        .prepare_cached(
            "
            SELECT 
                count(*)
            FROM 
                service_account_secrets 
            WHERE
                service_account_id IN (SELECT id FROM service_accounts WHERE vault_id = $1)
    ",
        )
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&vault_id]).await.unwrap();
    let count: i64 = rows[0].get(0);

    count
}

pub async fn count_secrets(config: &Config, email: &str) -> i64 {
    let client = config.db_pool.get().await.unwrap();
    let stmt = client
        .prepare_cached(
            "
            SELECT 
                id
            FROM 
                users 
            WHERE
                email = $1
    ",
        )
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&email]).await.unwrap();
    let user_id: i32 = rows[0].get(0);

    let stmt = client
        .prepare_cached(
            "
            SELECT 
                count(*)
            FROM 
                secrets 
            WHERE
                vault_id IN (SELECT vault_id FROM users_vaults WHERE USER_ID = $1)
    ",
        )
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&user_id]).await.unwrap();
    let count: i64 = rows[0].get(0);

    count
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
