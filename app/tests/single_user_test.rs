pub mod common;

use thirtyfour::prelude::*;

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
        let result = create_a_vault(&driver).await;

        let ten_seconds = std::time::Duration::from_secs(5);
        std::thread::sleep(ten_seconds);

        driver.quit().await?;

        result?;
    } else {
        // Always explicitly close the browser. There are no async destructors.
        driver.quit().await?;
        result?;
    }
    Ok(())
}

async fn create_a_vault(driver: &WebDriver) -> WebDriverResult<()> {
    let new_vault_button = driver.find_element(By::Id("new-vault")).await?;
    new_vault_button.click().await?;

    let submit_button = driver.find_element(By::Css("input[name='name']")).await?;
    submit_button.send_keys("My Vault").await?;

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
