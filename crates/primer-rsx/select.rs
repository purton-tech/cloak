#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelectSize {
    Default,
    Small,
    Large,
}

impl SelectSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            SelectSize::Default => "",
            SelectSize::Small => "sm",
            SelectSize::Large => "large",
        }
    }
}

impl Default for SelectSize {
    fn default() -> Self {
        SelectSize::Default
    }
}

#[derive(Props)]
pub struct SelectProps<'a> {
    children: Element<'a>,
    select_size: Option<SelectSize>,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub value: Option<&'a str>,
    pub label: Option<&'a str>,
    pub help_text: Option<&'a str>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

pub fn Select<'a>(cx: Scope<'a, SelectProps<'a>>) -> Element {
    let select_size = if cx.props.select_size.is_some() {
        cx.props.select_size.unwrap()
    } else {
        Default::default()
    };

    let value = cx.props.value.unwrap_or("");

    let class = select_size.to_string();

    cx.render(rsx!(
        match cx.props.label {
            Some(l) => cx.render(rsx!(
                label {
                    "{l}"
                }
            )),
            None => None
        }
        select {
            class: "{class}",
            value: "{value}",
            name: "{cx.props.name}",
            &cx.props.children
        }
        match cx.props.help_text {
            Some(l) => cx.render(rsx!(
                span {
                    class: "note mb-3",
                    "{l}"
                }
            )),
            None => None
        }
    ))
}
