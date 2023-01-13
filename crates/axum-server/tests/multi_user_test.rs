pub mod common;

use thirtyfour::{components::select::SelectElement, prelude::*};
use tokio::time::{sleep, Duration};

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

    let team_member = common::register_user(driver, config).await?;

    let account_owner = common::register_user(driver, config).await?;

    common::create_a_vault(driver).await?;

    common::add_secrets(
        driver,
        "ACCOUN_OWNER_SECRET",
        "1234-5679",
        "tbody > tr:last-child > td:first-child > ecdh-cipher",
    )
    .await?;

    set_profile_details(driver, &account_owner).await?;

    add_team_member(driver, &team_member, &account_owner, config).await?;

    sign_in_user(driver, &account_owner, config).await?;

    // Pull up the vault we created
    common::select_first_vault(driver).await?;

    add_member_to_vault(driver, &team_member).await?;

    // Log back in as the member
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
    // Go to sign in page
    driver.get(format!("{}/auth/sign_in", &config.host)).await?;

    // Sign in someone
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

// Before we ivite people we have to have a team name and set our own name
async fn set_profile_details(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Click on the profile button
    let path = format!("//span[text()='{}']", email);
    driver.find_element(By::XPath(&path)).await?.click().await?;

    driver
        .find_element(By::LinkText("Profile"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::Css("input[name='first_name']"))
        .await?
        .send_keys("David")
        .await?;

    driver
        .find_element(By::Css("input[name='last_name']"))
        .await?
        .send_keys("Jason")
        .await?;

    driver
        .find_element(By::XPath("//button[text()='Update Profile']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Now set the org name
    driver
        .find_element(By::LinkText("Team Members"))
        .await?
        .click()
        .await?;

    driver
        .find_element(By::XPath("//button[text()='Edit Name']"))
        .await?
        .click()
        .await?;

    // Wait for the form to appear
    driver
        .query(By::Css("input[name='name']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    driver
        .find_element(By::Css("input[name='name']"))
        .await?
        .send_keys("Testing Team")
        .await?;

    driver
        .find_element(By::XPath("//button[text()='Set Team Name']"))
        .await?
        .click()
        .await?;

    Ok(())
}

async fn add_member_to_vault(driver: &WebDriver, email: &str) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::LinkText("Members"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::XPath("//button[text()='Add Member']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    let vault_selector = driver.find_element(By::Css("select:first-of-type")).await?;
    let select = SelectElement::new(&vault_selector).await?;
    select.select_by_exact_text(email).await?;

    // Check the development environment
    let dev_label = driver
        .find_element(By::Css("label[for='Development']"))
        .await?;
    dev_label.click().await?;

    driver
        .find_element(By::XPath("//button[text()='Add User to Vault']"))
        .await?
        .click()
        .await?;

    Ok(())
}

async fn add_team_member(
    driver: &WebDriver,
    team_member: &str,
    team_owner: &str,
    config: &common::Config,
) -> WebDriverResult<()> {
    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    // Click on the side menu
    driver
        .find_element(By::LinkText("Team Members"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::XPath("//button[text()='Invite New Team Member']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    driver
        .find_element(By::Css("input[name='email']"))
        .await?
        .send_keys(team_member)
        .await?;

    driver
        .find_element(By::Css("input[name='first_name']"))
        .await?
        .send_keys("Trevor")
        .await?;

    driver
        .find_element(By::Css("input[name='last_name']"))
        .await?
        .send_keys("Invitable")
        .await?;

    driver
        .find_element(By::XPath("//button[text()='Send Invitation']"))
        .await?
        .click()
        .await?;

    // Stop stale element error
    sleep(Duration::from_millis(1000)).await;

    let table_cell = driver
        .find_element(By::XPath("//tbody/tr[last()]/td[1]/span"))
        .await?;

    assert_eq!(table_cell.text().await?, "Trevor Invitable");

    // Get the invite from mailhog
    let invitation_url = get_invite_url_from_email(config).await?;

    sign_in_user(driver, team_member, config).await?;

    // Accept the invitation
    driver.get(invitation_url).await?;

    let table_cell = driver
        .find_element(By::XPath("//tbody/tr[1]/td[2]"))
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
