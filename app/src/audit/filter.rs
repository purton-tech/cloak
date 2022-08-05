use crate::authentication::Authentication;
use crate::cornucopia::queries;
use crate::cornucopia::types;
use crate::errors::CustomError;
use axum::{
    extract::{Extension, Form, Path},
    response::Html,
};
use deadpool_postgres::Pool;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Filter {
    pub id: i32,
    pub user: i32,
    pub access_type: u32,
    pub action: u32,
}

impl Filter {
    
    pub fn get_id(&self) -> Option<i32> {
        match self.id {
            0 => None,
            n => Some(n)
        }
    }

    pub fn get_user(&self) -> Option<i32> {
        match self.user {
            0 => None,
            n => Some(n)
        }
    }

    pub fn convert_to_access_type(&self) -> Option<types::public::AuditAccessType> {
        match self.access_type {
            0 => None,
            1 => Some(types::public::AuditAccessType::Web),
            2 => Some(types::public::AuditAccessType::CLI),
            _ => Some(types::public::AuditAccessType::ServiceAccount)
        }
    }

    pub fn convert_to_action(&self) -> Option<types::public::AuditAction> {
        match self.action {
            0 => None,
            1 => Some(types::public::AuditAction::AddMember),
            2 => Some(types::public::AuditAction::DeleteMember),
            3 => Some(types::public::AuditAction::AddSecret),
            4 => Some(types::public::AuditAction::DeleteSecret),
            5 => Some(types::public::AuditAction::AccessSecrets),
            6 => Some(types::public::AuditAction::NewServiceAccount),
            7 => Some(types::public::AuditAction::DeleteServiceAccount),
            8 => Some(types::public::AuditAction::ConnectServiceAccount),
            9 => Some(types::public::AuditAction::CreateInvite),
            10 => Some(types::public::AuditAction::RemoveTeamMember),
            11 => Some(types::public::AuditAction::CreateVault),
            _ => Some(types::public::AuditAction::DeleteVault)
        }
    }
}

pub async fn filter(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
    Form(filter_form): Form<Filter>,
) -> Result<Html<&'static str>, CustomError> {
    // Create a transaction and setup RLS
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;
    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let user = queries::users::get()
        .bind(&transaction, &(current_user.user_id as i32))
        .one()
        .await?;
    let initials = crate::layout::initials(&user.email, user.first_name, user.last_name);

    let team = queries::organisations::organisation()
        .bind(&transaction, &organisation_id)
        .one()
        .await?;

    let team_users = queries::organisations::get_users()
        .bind(&transaction, &organisation_id)
        .all()
        .await?;

    let audits = queries::audit::audit()
        .bind(
            &transaction,
            &filter_form.get_id(),
            &filter_form.convert_to_action(),
            &filter_form.convert_to_access_type(),
            &filter_form.get_user(),
            &organisation_id,
            &(super::PAGE_SIZE + 1),
        )
        .all()
        .await?;

    Ok(crate::render(|buf| {
        crate::templates::audit::index_html(buf, &initials, audits, team_users, &team, false)
    }))
}
