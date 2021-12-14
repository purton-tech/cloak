use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::PgPool;

pub async fn index(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    authentication: Authentication,
) -> Result<Html<String>, CustomError> {
    let secrets = models::Secret::get_all(pool, authentication.user_id, id).await?;

    let page = SecretsPage { secrets };

    crate::layout::layout("Home", &page.to_string())
}

markup::define! {
    SecretsPage(secrets: Vec<models::Secret>) {
        div.m_card {
            div.header {
                span { "Secrets" }
            }
            div.body {
                table.m_table {
                    thead {
                        tr {
                            th { "Name" }
                            th { "Updated" }
                            th { "Created" }
                        }
                    }
                    tbody {
                        @for secret in secrets {
                            tr {
                                td { {secret.name} }
                                td { "Updated" }
                                td { "Created" }
                            }
                        }
                    }
                }
            }
        }
    }
}
