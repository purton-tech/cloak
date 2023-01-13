#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct DrawerProps<'a> {
    trigger_id: &'a str,
    label: &'a str,
    children: Element<'a>,
    component_name: Option<&'a str>,
}

pub fn Drawer<'a>(cx: Scope<'a, DrawerProps<'a>>) -> Element {
    let drawer_wc = if let Some(component_name) = cx.props.component_name {
        format!(
            "<{} class='side_drawer d-flex flex-column' label='{}' id='{}'>",
            component_name, cx.props.label, cx.props.trigger_id
        )
    } else {
        format!(
            "<side-drawer class='side_drawer d-flex flex-column' label='{}' id='{}'>",
            cx.props.label, cx.props.trigger_id
        )
    };

    let drawer_close_name = if let Some(component_name) = cx.props.component_name {
        component_name
    } else {
        "side-drawer"
    };

    cx.render(rsx!(
        {
            LazyNodes::new(|f| f.text(format_args!("{drawer_wc}")))
        }
        {
            &cx.props.children
        }
        {
            LazyNodes::new(|f| f.text(format_args!("</{drawer_close_name}>")))
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
