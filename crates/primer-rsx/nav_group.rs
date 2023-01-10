#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct NavGroupProps<'a> {
    heading: &'a str,
    content: Element<'a>,
}

pub fn NavGroup<'a>(cx: Scope<'a, NavGroupProps<'a>>) -> Element {
    cx.render(rsx!(
        ul {
            role: "list",
            class: "ActionListWrap",
            li {
                class: "ActionList-sectionDivider",
                h3 {
                    class: "ActionList-sectionDivider-title",
                    "{cx.props.heading}"
                }
            }
            &cx.props.content
        }
    ))
}
