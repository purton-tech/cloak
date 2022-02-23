pub mod common;

use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_single_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = single_user(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn single_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(&config.host).await?;

    let email = common::register_user(driver, config).await?;

    dbg!(&email);

    common::create_a_vault(driver).await?;

    common::add_secrets(
        driver,
        "PRIVATE_KEY",
        "1234-5678",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    common::add_secrets(
        driver,
        "PRIVATE_KEY2",
        "1234-5678",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    let count = common::count_secrets(config, &email)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    assert_eq!(count, 2);

    common::add_service_account(driver).await?;

    let count = common::count_service_account_secrets(config, &email)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    assert_eq!(count, 2);

    common::select_first_vault(driver).await?;

    common::add_secrets(
        driver,
        "WITH_SERVICE_ACCOUNT",
        "1234-5679",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    let count = common::count_secrets(config, &email)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    assert_eq!(count, 3);

    let count = common::count_service_account_secrets(config, &email)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    assert_eq!(count, 3);

    Ok(())
}
