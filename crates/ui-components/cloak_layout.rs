#![allow(non_snake_case)]
use super::logout_form::LogoutForm;
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::{AppLayout, NavGroup, NavItem};

#[derive(PartialEq, Eq, Debug)]
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

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Props)]
pub struct CloakLayoutProps<'a> {
    selected_item: SideBar,
    title: &'a str,
    header: Element<'a>,
    children: Element<'a>,
    team_id: i32,
    vault_id: Option<i32>,
}

pub fn CloakLayout<'a>(cx: Scope<'a, CloakLayoutProps<'a>>) -> Element {
    cx.render(rsx! {
        AppLayout {
            title: cx.props.title,
            css_href1: primer_view_components_css.name,
            css_href2: index_css.name,
            js_href: index_js.name,
            fav_icon_src: favicon_svg.name,
            header: cx.render(rsx!(
                &cx.props.header
            ))
            sidebar: cx.render(rsx!(
                NavGroup {
                    heading: "Vaults",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Vaults.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::vaults::index_route(cx.props.team_id),
                            icon: nav_api_keys_svg.name,
                            title: "Vaults"
                        }
                        if let Some(vault_id) = cx.props.vault_id {
                            cx.render(rsx!(
                                NavItem {
                                    id: SideBar::Secrets.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::secrets::index_route(vault_id, cx.props.team_id),
                                    icon: nav_api_keys_svg.name,
                                    title: "Secrets"
                                }
                                NavItem {
                                    id: SideBar::Members.to_string(),
                                    selected_item_id: cx.props.selected_item.to_string(),
                                    href: super::routes::vaults::index_route(cx.props.team_id),
                                    icon: nav_api_keys_svg.name,
                                    title: "Members"
                                }
                            ))
                        } else {
                            None
                        }
                        NavItem {
                            id: SideBar::ServiceAccounts.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::service_accounts::index_route(cx.props.team_id),
                            icon: nav_api_keys_svg.name,
                            title: "Service Accounts"
                        }
                        NavItem {
                            id: SideBar::Audit.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::audit::index_route(cx.props.team_id),
                            icon: nav_api_keys_svg.name,
                            title: "Audit Trail"
                        }
                    ))
                }
                NavGroup {
                    heading: "Collaboration",
                    content:  cx.render(rsx!(
                        NavItem {
                            id: SideBar::Team.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::team::index_route(cx.props.team_id),
                            icon: nav_members_svg.name,
                            title: "Team Members"
                        }
                        NavItem {
                            id: SideBar::Switch.to_string(),
                            selected_item_id: cx.props.selected_item.to_string(),
                            href: super::routes::team::switch_route(cx.props.team_id),
                            icon: nav_teams_svg.name,
                            title: "Your Teams"
                        }
                    ))
                }
            ))
            sidebar_header: cx.render(rsx!(
                {
                    LazyNodes::new(|f| f.text(format_args!("<turbo-frame id='teams-popup' class='width-full' src='{}'></turbo-frame>", 
                        super::routes::team::teams_popup_route(cx.props.team_id))))
                }
            ))
            sidebar_footer: cx.render(rsx!(
                {
                    LazyNodes::new(|f| f.text(format_args!("<turbo-frame id='profile-popup' class='width-full' src='{}'></turbo-frame>",
                    super::routes::profile::profile_popup_route(cx.props.team_id))))
                }
            )),
            &cx.props.children
            {
                LazyNodes::new(|f| f.text(format_args!("<snack-bar></snack-bar>")))
            }
        }
        LogoutForm {}
    })
}
