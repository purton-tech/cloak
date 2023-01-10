#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct TimeLineProps<'a> {
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn TimeLine<'a>(cx: Scope<'a, TimeLineProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("TimelineItem {}", class);

    cx.render(rsx!(
        div {
            class: "{class}",
            {&cx.props.children}
        }
    ))
}

#[derive(Props)]
pub struct TimeLineBadgeProps<'a> {
    image_src: &'a str,
}

pub fn TimeLineBadge<'a>(cx: Scope<'a, TimeLineBadgeProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "TimelineItem-badge",
            img {
                src: "{cx.props.image_src}",
                width: "16"
            }
        }
    ))
}

#[derive(Props)]
pub struct TimeLineBodyProps<'a> {
    children: Element<'a>,
}

pub fn TimeLineBody<'a>(cx: Scope<'a, TimeLineBodyProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "TimelineItem-body",
            &cx.props.children
        }
    ))
}
