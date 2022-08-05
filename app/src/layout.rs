use crate::errors::CustomError;
use axum::http::Response;
use hyper::{Body, StatusCode};

#[derive(PartialEq, Eq)]
pub enum SideBar {
    None,
    Audit,
    Vaults,
    Secrets,
    Members,
    ServiceAccounts,
    Team,
    Profile,
    Switch,
}

pub fn get_menu_class(side_bar: &SideBar, selected_sidebar: &SideBar, sub_menu: bool) -> String {
    let selected = selected_sidebar == side_bar;
    match (selected, sub_menu) {
        (true, true) => "selected submenu",
        (true, false) => "selected",
        (false, true) => "submenu",
        (false, false) => "",
    }
    .to_string()
}

pub fn name(email: String, first_name: Option<String>, last_name: Option<String>) -> String {
    if let (Some(first_name), Some(last_name)) = (first_name, last_name) {
        return format!("{} {}", first_name, last_name);
    }

    email
}

pub fn initials(email: &str, first_name: Option<String>, last_name: Option<String>) -> String {
    if let (Some(first_name), Some(last_name)) = (first_name, last_name) {
        let i1 = first_name.chars().next();
        let i2 = last_name.chars().next();

        let intials = match (i1, i2) {
            (Some(first_initial), Some(last_initial)) => {
                format!("{}{}", first_initial, last_initial)
            }
            (Some(first_initial), None) => format!("{}", first_initial),
            _ => "".to_string(),
        };
        return intials;
    }

    return email.chars().next().unwrap().to_string().to_uppercase();
}

pub fn redirect_and_snackbar(
    url: &str,
    message: &'static str,
) -> Result<Response<Body>, CustomError> {
    let builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("location", url)
        .header("set-cookie", format!("flash_aargh={}; Max-Age=6", message))
        .body(Body::empty());
    let response =
        builder.map_err(|_| CustomError::FaultySetup("Could not build redirect".to_string()))?;
    Ok(response)
}
