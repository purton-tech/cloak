use crate::cloak_layout::{CloakLayout, SideBar};
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
struct AuditProps {
    organisation_id: i32,
}

pub fn index(organisation_id: i32) -> String {
    fn app(cx: Scope<AuditProps>) -> Element {
        cx.render(rsx! {

            CloakLayout {
                selected_item: SideBar::Audit,
                team_id: cx.props.organisation_id,
                title: "Dashboard"
                header: cx.render(rsx!(
                    h3 { "Dashboard" }
                ))
                div {

                }
            }
        })
    }

    let mut app = VirtualDom::new_with_props(app, AuditProps { organisation_id });
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
