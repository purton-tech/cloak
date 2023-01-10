#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct NavItemProps<'a> {
    href: String,
    icon: &'a str,
    title: &'a str,
    selected_item_id: Option<String>,
    id: Option<String>,
}

pub fn NavItem<'a>(cx: Scope<'a, NavItemProps<'a>>) -> Element {
    let mut class = "ActionListItem";
    if let (Some(id), Some(selected_item_id)) = (&cx.props.id, &cx.props.selected_item_id) {
        if id == selected_item_id {
            class = "ActionListItem ActionListItem--navActive";
        }
    }
    cx.render(rsx!(
        li {
            role: "listitem",
            class: "{class}",
            a {
                href: "{cx.props.href}",
                class: "ActionListContent ActionListContent--visual16",
                span {
                    class: "ActionListItem-visual ActionListItem-visual--leading",
                    img {
                        width: "16",
                        height: "16",
                        src: "{cx.props.icon}"
                    }
                }
                span {
                    class: "ActionListItem-label",
                    "{cx.props.title}"
                }
            }
        }
    ))
}
