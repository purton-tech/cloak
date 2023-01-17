#![allow(non_snake_case)]
use db::AuditAccessType;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    pub access_type: &'a AuditAccessType,
}

pub fn AuditAccessType<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    match cx.props.access_type {
        AuditAccessType::CLI => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "CLI"
            }
        )),
        AuditAccessType::ServiceAccount => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Attention,
                "Service Account"
            }
        )),
        AuditAccessType::Web => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Attention,
                "Web App"
            }
        )),
    }
}
