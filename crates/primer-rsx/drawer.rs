#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct DrawerProps<'a> {
    trigger_id: &'a str,
    label: &'a str,
    children: Element<'a>,
}

pub fn Drawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    let drawer_wc = format!(
        "<side-drawer class='side_drawer d-flex flex-column' label='{}' id='{}'>",
        cx.props.label, cx.props.trigger_id
    );
    cx.render(rsx!(
        {
            LazyNodes::new(|f| f.text(format_args!("{drawer_wc}")))
        }
        {
            &cx.props.children
        }
        {
            LazyNodes::new(|f| f.text(format_args!("</side-drawer>")))
        }
    ))
}

#[derive(Props)]
pub struct DrawerFooterProps<'a> {
    children: Element<'a>,
}

pub fn DrawerFooter<'a>(cx: Scope<'a, DrawerFooterProps<'a>>) -> Element {
    cx.render(rsx!(
        template {
            "slot": "footer",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct DrawerBodyProps<'a> {
    children: Element<'a>,
}

pub fn DrawerBody<'a>(cx: Scope<'a, DrawerBodyProps<'a>>) -> Element {
    cx.render(rsx!(cx.render(rsx!(
        template {
            "slot": "body",
            &cx.props.children
        }
    ))))
}
