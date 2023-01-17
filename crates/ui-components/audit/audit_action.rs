#![allow(non_snake_case)]
use db::AuditAction;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    pub audit_action: &'a AuditAction,
}

pub fn AuditAction<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    match cx.props.audit_action {
        AuditAction::AccessSecrets => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Access Secrets"
            }
        )),
        AuditAction::AddMember => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Add Member"
            }
        )),
        AuditAction::AddSecret => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Add Secret"
            }
        )),
        AuditAction::ConnectServiceAccount => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Connect Service Account"
            }
        )),
        AuditAction::CreateInvite => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Create Invite"
            }
        )),
        AuditAction::CreateVault => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Create Vault"
            }
        )),
        AuditAction::DeleteMember => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Delete Member"
            }
        )),
        AuditAction::DeleteSecret => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Delete Secret"
            }
        )),
        AuditAction::DeleteServiceAccount => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Delete Service Account"
            }
        )),
        AuditAction::DeleteVault => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Delete Vault"
            }
        )),
        AuditAction::NewServiceAccount => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "New Service Account"
            }
        )),
        AuditAction::RemoveTeamMember => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                "Remove Team Member"
            }
        )),
    }
}
