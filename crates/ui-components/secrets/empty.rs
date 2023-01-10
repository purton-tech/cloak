#![allow(non_snake_case)]
use crate::cloak_layout::{CloakLayout, SideBar};
use assets::files::empty_api_keys_svg;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct EmptySecretsProps {
    organisation_id: i32,
}

pub fn EmptySecrets(cx: Scope<EmptySecretsProps>) -> Element {
    cx.render(rsx!(
        CloakLayout {
            selected_item: SideBar::Secrets,
            team_id: cx.props.organisation_id,
            title: "Secrets"
            header: cx.render(rsx!(
                h3 { "Secrets" }
            ))
            BlankSlate {
                heading: "Looks like you don't have any API keys",
                visual: empty_api_keys_svg.name,
                description: "API Keys allow you to access our programming interface",
                primary_action_drawer: ("New API Key", "create-api-key")
            }
        }
    ))
}
