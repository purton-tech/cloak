pub mod common;

use std::time::Duration;

use thirtyfour::{components::select::SelectElement, prelude::*};
use tokio::time::sleep;

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

async fn audit_filter(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    let audit_link = driver.find_element(By::LinkText("Audit Trail")).await?;
    audit_link.click().await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    let filter_button = driver
        .find_element(By::XPath("//button[text()='Filter']"))
        .await?;
    filter_button.click().await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    let user_selector = driver.find_element(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&user_selector).await?;
    select.select_by_exact_text(email).await?;

    driver
        .find_element(By::XPath("//button[text()='Apply Filter']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // See it in the search results
    let table_cell = driver
        .find_element(By::XPath("//tbody/tr[last()]/td[2]"))
        .await?;

    assert_eq!(table_cell.text().await?, email);

    Ok(())
}

async fn single_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(format!("{}/auth/sign_up", &config.host)).await?;

    println!("Testing : register_user");

    let email = common::register_user(driver, config).await?;

    println!("Testing : create_a_vault");

    common::create_a_vault(driver).await?;

    println!("Testing : add_secrets");

    common::add_secrets(
        driver,
        "PRIVATE_KEY",
        "1234-5678",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    println!("Testing : add_secrets");

    common::add_secrets(
        driver,
        "PRIVATE_KEY2",
        "1234-5678",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    println!("Testing : count_secrets");

    let count = common::count_secrets(config, &email).await;

    assert_eq!(count, 2);

    println!("Testing : add_service_account");

    common::add_service_account(driver).await?;

    println!("Testing : count_service_account_secrets");

    let count = common::count_service_account_secrets(config, &email).await;

    assert_eq!(count, 2);

    println!("Testing : select_first_vault");

    common::select_first_vault(driver).await?;

    println!("Testing : add_secrets");

    common::add_secrets(
        driver,
        "WITH_SERVICE_ACCOUNT",
        "1234-5679",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    println!("Testing : count_secrets");

    let count = common::count_secrets(config, &email).await;

    assert_eq!(count, 3);

    println!("Testing : count_service_account_secrets");

    let count = common::count_service_account_secrets(config, &email).await;

    assert_eq!(count, 3);

    println!("Testing : audit_filter");

    audit_filter(driver, &email).await?;

    Ok(())
}
