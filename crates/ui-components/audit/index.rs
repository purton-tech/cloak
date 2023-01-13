use crate::cloak_layout::{CloakLayout, SideBar};
use db::{Audit, Member};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct AuditProps {
    team_users: Vec<Member>,
    audits: Vec<Audit>,
    organisation_id: i32,
    reset_search: bool,
}

pub fn index(organisation_id: i32, team_users: Vec<Member>, audits: Vec<Audit>) -> String {
    fn app(cx: Scope<AuditProps>) -> Element {
        cx.render(rsx! {

            CloakLayout {
                selected_item: SideBar::Audit,
                team_id: cx.props.organisation_id,
                title: "Audit Trail"
                header: cx.render(rsx!(
                    h3 { "Audit Trail" }
                    Button {
                        drawer_trigger: super::filter::DRAW_TRIGGER,
                        button_scheme: ButtonScheme::Default,
                        "Filter"
                    }
                ))
                super::table::AuditTable {
                    audits: &cx.props.audits
                }
                super::filter::FilterDrawer {
                    team_users: cx.props.team_users.clone(),
                    organisation_id: cx.props.organisation_id,
                    reset_search: cx.props.reset_search,
                    submit_action: crate::routes::audit::index_route(cx.props.organisation_id)
                }
            }
        })
    }

    let mut app = VirtualDom::new_with_props(
        app,
        AuditProps {
            team_users,
            audits,
            reset_search: true,
            organisation_id,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
