pub mod common;

use thirtyfour::{components::select::SelectElement, prelude::*};

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

    let filter_button = driver
        .find_element(By::XPath("//button[text()='Filter']"))
        .await?;
    filter_button.click().await?;

    let user_selector = driver.find_element(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&user_selector).await?;
    select.select_by_exact_text(email).await?;

    let submit_button = driver
        .find_element(By::Css(".a_button.auto.success"))
        .await?;
    submit_button.click().await?;

    // See it in the search results
    let table_cell = driver
        .find_element(By::XPath(
            "//table[@class='m_table audit_table']/tbody/tr[last()]/td[2]",
        ))
        .await?;

    assert_eq!(table_cell.text().await?, email);

    Ok(())
}

async fn single_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(&config.host).await?;

    let email = common::register_user(driver, config).await?;

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

    let count = common::count_secrets(config, &email).await;

    assert_eq!(count, 2);

    common::add_service_account(driver).await?;

    let count = common::count_service_account_secrets(config, &email).await;

    assert_eq!(count, 2);

    common::select_first_vault(driver).await?;

    common::add_secrets(
        driver,
        "WITH_SERVICE_ACCOUNT",
        "1234-5679",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    let count = common::count_secrets(config, &email).await;

    assert_eq!(count, 3);

    let count = common::count_service_account_secrets(config, &email).await;

    assert_eq!(count, 3);

    audit_filter(driver, &email).await?;

    Ok(())
}
