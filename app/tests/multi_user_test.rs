pub mod common;

use thirtyfour::{components::select::SelectElement, prelude::*};

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_multi_user() -> WebDriverResult<()> {
    let config = common::Config::new().await;

    let driver = config.get_driver().await?;

    let result = multi_user(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn multi_user(driver: &WebDriver, config: &common::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(&config.host).await?;

    let team_member = common::register_user(driver, config).await?;

    // Go to home page
    driver.get(&config.host).await?;

    let account_owner = common::register_user(driver, config).await?;

    common::create_a_vault(driver).await?;

    common::add_secrets(
        driver,
        "ACCOUN_OWNER_SECRET",
        "1234-5679",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    add_team_member(driver, &team_member, &account_owner, config).await?;

    sign_in_user(driver, &account_owner, config).await?;

    // Pull up the vault we created
    common::select_first_vault(driver).await?;

    add_member_to_vault(driver, &team_member).await?;

    // Log back in as the member
    driver.get(&config.host).await?;
    sign_in_user(driver, &team_member, config).await?;

    // Add a service account
    common::add_service_account(driver).await?;

    let count = common::count_service_account_secrets(config, &team_member).await;

    assert_eq!(count, 1);

    Ok(())
}

async fn sign_in_user(
    driver: &WebDriver,
    email: &str,
    config: &common::Config,
) -> WebDriverResult<()> {
    // Go to home page
    driver.get(&config.host).await?;

    // Register someone
    driver
        .find_element(By::LinkText("SIGN IN"))
        .await?
        .click()
        .await?;
    driver
        .find_element(By::Id("email"))
        .await?
        .send_keys(email)
        .await?;
    driver
        .find_element(By::Id("password"))
        .await?
        .send_keys(email)
        .await?;
    driver
        .find_element(By::Css("button[type='submit']"))
        .await?
        .click()
        .await?;

    // OTP Code
    // Wait for page to load as code might not be in database yet.
    driver.find_element(By::Id("code")).await?;

    common::force_otp(config).await;

    driver.get(format!("{}/auth/decrypt", config.host)).await?;

    Ok(())
}

async fn add_member_to_vault(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    let sa_link = driver.find_element(By::LinkText("Members")).await?;
    sa_link.click().await?;

    let new_user_button = driver.find_element(By::Id("add-member")).await?;
    new_user_button.click().await?;

    let vault_selector = driver.find_element(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&vault_selector).await?;
    select.select_by_exact_text(email).await?;

    // Check the development environment
    let dev_label = driver.find_element(By::Css("label[for='Development']")).await?;
    dev_label.click().await?;

    let submit_button = driver
        .find_element(By::Css(".a_button.auto.success"))
        .await?;
    submit_button.click().await?;

    Ok(())
}

async fn add_team_member(
    driver: &WebDriver,
    team_member: &str,
    team_owner: &str,
    config: &common::Config,
) -> WebDriverResult<()> {
    let sa_link = driver.find_element(By::LinkText("Team")).await?;
    sa_link.click().await?;

    let new_user_button = driver.find_element(By::Id("invite-user")).await?;
    new_user_button.click().await?;

    let name_field = driver.find_element(By::Css("input[name='email']")).await?;
    name_field.send_keys(team_member).await?;

    let submit_button = driver
        .find_element(By::Css(".a_button.auto.success"))
        .await?;
    submit_button.click().await?;

    let table_cell = driver
        .find_element(By::XPath(
            "//table[@class='m_table team_table']/tbody/tr[last()]/td[1]/span",
        ))
        .await?;

    assert_eq!(table_cell.text().await?, "Invited");

    // Get the invite from mailhog
    let invitation_url = get_invite_url_from_email(config).await?;

    sign_in_user(driver, team_member, config).await?;

    // Accept the invitation
    driver.get(invitation_url).await?;

    let table_cell = driver
        .find_element(By::XPath(
            "//table[@class='m_table memberships']/tbody/tr[last()]/td[2]",
        ))
        .await?;

    assert_eq!(table_cell.text().await?, team_owner);

    Ok(())
}

async fn get_invite_url_from_email(config: &common::Config) -> WebDriverResult<String> {
    let body: String = reqwest::get(config.mailhog_url.clone())
        .await?
        .text()
        .await?;

    let url: Vec<&str> = body.split("Click ").collect();
    let url: Vec<&str> = url[1].split(" to accept the invite").collect();

    // Emails are generally tructed to 78 columns. sigh.
    let url = quoted_printable::decode(url[0], quoted_printable::ParseMode::Robust).unwrap();
    let url = String::from_utf8(url).unwrap();

    let url = url.replace("\\u0026", "&");
    let url = url.replace("=\\r\\n", "");
    

    dbg!(&url);

    Ok(url)
}
