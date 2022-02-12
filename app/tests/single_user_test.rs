pub mod common;

use thirtyfour::{components::select::SelectElement, prelude::*};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn single_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(&config.host).await?;

    let result = register_user(&driver, &config).await;
    if result.is_ok() {
        result?;
    } else {
        driver.quit().await?;
        result?;
        return Ok(());
    }

    let result = create_a_vault(&driver).await;
    if result.is_ok() {
        result?;
    } else {
        driver.quit().await?;
        result?;
        return Ok(());
    }

    let result = add_secrets(
        &driver,
        "PRIVATE_KEY",
        "1234-5678",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await;
    if result.is_ok() {
        result?;
    } else {
        driver.quit().await?;
        result?;
        return Ok(());
    }

    let result = add_secrets(
        &driver,
        "PRIVATE_KEY2",
        "1234-5678",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await;
    if result.is_ok() {
        result?;
    } else {
        driver.quit().await?;
        result?;
        return Ok(());
    }

    let result = add_service_account(&driver).await;
    if result.is_ok() {
        result?;
    } else {
        let five_secs = std::time::Duration::from_secs(5);
        std::thread::sleep(five_secs);
        driver.quit().await?;
        result?;
        return Ok(());
    }

    let five_secs = std::time::Duration::from_secs(5);
    std::thread::sleep(five_secs);

    driver.quit().await?;

    Ok(())
}

async fn add_service_account(driver: &WebDriver) -> WebDriverResult<()> {
    let sa_link = driver
        .find_element(By::LinkText("Service Accounts"))
        .await?;
    sa_link.click().await?;

    let new_account_button = driver.find_element(By::Id("new-account")).await?;
    new_account_button.click().await?;

    let name_field = driver.find_element(By::Css("input[name='name']")).await?;
    name_field.send_keys("My Dev Machine").await?;

    let submit_button = driver
        .find_element(By::Css(".a_button.auto.success"))
        .await?;
    submit_button.click().await?;

    let attach_link = driver.find_element(By::LinkText("Attach to Vault")).await?;
    attach_link.click().await?;

    let vault_selector = driver.find_element(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&vault_selector).await?;
    select.select_by_exact_text("My Vault").await?;

    // Connect this account
    let connect_button = driver
        .find_element(By::XPath("//button[text()='Connect to Vault']"))
        .await?;
    connect_button.click().await?;

    Ok(())
}

async fn add_secrets(
    driver: &WebDriver,
    name: &str,
    value: &str,
    selector: &str,
) -> WebDriverResult<()> {
    let new_secret_button = driver.find_element(By::Id("new-secret")).await?;
    new_secret_button.click().await?;

    let name_field = driver.find_element(By::Css("input[name='name']")).await?;
    name_field.send_keys(name).await?;

    let secret_field = driver.find_element(By::Css("input[name='secret']")).await?;
    secret_field.send_keys(value).await?;

    // Wait for it to open
    let submit_button = driver
        .find_element(By::Css(".a_button.auto.success"))
        .await?;
    submit_button.click().await?;

    // Make sure the drawer is gone
    let pause = std::time::Duration::from_millis(500);
    std::thread::sleep(pause);

    let ecdh_cipher = driver.find_element(By::Css(selector)).await?;
    assert_eq!(ecdh_cipher.text().await?, name);

    Ok(())
}

async fn create_a_vault(driver: &WebDriver) -> WebDriverResult<()> {
    let new_vault_button = driver.find_element(By::Id("new-vault")).await?;
    new_vault_button.click().await?;

    let name_field = driver.find_element(By::Css("input[name='name']")).await?;
    name_field.send_keys("My Vault").await?;

    // Wait for it to open
    let submit_button = driver
        .find_element(By::Css(".a_button.auto.success"))
        .await?;
    submit_button.click().await?;

    let vault_card = driver.find_element(By::Css(".vault-card")).await?;
    vault_card.click().await?;

    Ok(())
}

async fn register_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let _email = common::register_random_user(driver).await?;

    common::force_otp(config)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    driver.get(format!("{}/auth/decrypt", config.host)).await?;

    Ok(())
}
