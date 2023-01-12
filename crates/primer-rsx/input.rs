#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    Text,
    Email,
    Password,
}

impl InputType {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
        }
    }
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Text
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputSize {
    Default,
    Small,
    Large,
}

impl InputSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputSize::Default => "",
            InputSize::Small => "sm",
            InputSize::Large => "large",
        }
    }
}

impl Default for InputSize {
    fn default() -> Self {
        InputSize::Default
    }
}

#[derive(Props)]
pub struct InputProps<'a> {
    input_type: Option<InputType>,
    input_size: Option<InputSize>,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub label_class: Option<&'a str>,
    pub value: Option<&'a str>,
    pub label: Option<&'a str>,
    pub help_text: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    let input_type = if cx.props.input_type.is_some() {
        cx.props.input_type.unwrap()
    } else {
        Default::default()
    };

    let input_size = if cx.props.input_size.is_some() {
        cx.props.input_size.unwrap()
    } else {
        Default::default()
    };

    let value = cx.props.value.unwrap_or("");
    let input_type = input_type.to_string();
    let input_size = input_size.to_string();

    let required = if cx.props.required.is_some() {
        cx.props.required.unwrap()
    } else {
        false
    };

    let disabled = if cx.props.disabled.is_some() {
        cx.props.disabled.unwrap()
    } else {
        false
    };

    let readonly = if cx.props.readonly.is_some() {
        cx.props.readonly.unwrap()
    } else {
        false
    };

    let placeholder = if cx.props.placeholder.is_some() {
        cx.props.placeholder.unwrap()
    } else {
        ""
    };

    let label_class = if let Some(label_class) = cx.props.label_class {
        label_class
    } else {
        ""
    };

    let id = if let Some(id) = cx.props.id { id } else { "" };

    let input_class = format!("{} {}", input_type, input_size);

    cx.render(rsx!(
        match cx.props.label {
            Some(l) => cx.render(rsx!(
                label {
                    class: "{label_class}",
                    "{l}"
                }
            )),
            None => None
        }
        input {
            id: "{id}",
            class: "{input_class}",
            value: "{value}",
            name: "{cx.props.name}",
            required: "{required}",
            disabled: "{disabled}",
            readonly: "{readonly}",
            placeholder: "{placeholder}",
            "type": "{input_type}"
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
