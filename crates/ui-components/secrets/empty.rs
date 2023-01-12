#![allow(non_snake_case)]
use crate::cloak_layout::{CloakLayout, SideBar};
use assets::files::empty_api_keys_svg;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct EmptySecretsProps {
    organisation_id: i32,
    vault_id: i32,
}

pub fn EmptySecrets(cx: Scope<EmptySecretsProps>) -> Element {
    cx.render(rsx!(
        CloakLayout {
            selected_item: SideBar::Secrets,
            team_id: cx.props.organisation_id,
            vault_id: cx.props.vault_id,
            title: "Secrets"
            header: cx.render(rsx!(
                h3 { "Secrets" }
            ))
            BlankSlate {
                heading: "This vault doesn't have any secrets yet",
                visual: empty_api_keys_svg.name,
                description: "Create your first secret and add it to the vault",
                primary_action_drawer: ("Create A New Secret", super::new_secret::DRAW_TRIGGER)
            }
        }
    ))
}
