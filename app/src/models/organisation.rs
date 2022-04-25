use crate::cornucopia::queries;
use crate::errors::CustomError;
use deadpool_postgres::Pool;

pub struct Organisation {
    pub id: i32,
    pub name: Option<String>,
}

impl Organisation {

    // Add an invited user to the organisation
    pub async fn add_user_dangerous(
        pool: &Pool,
        email: &str,
        organisation_id: i32,
    ) -> Result<(), CustomError> {
        let client = pool.get().await?;

        let user = queries::users::get_by_email_dangerous(&client, &email).await?;

        queries::organisations::add_user_to_organisation(&client, &user.id, &organisation_id)
            .await?;

        Ok(())
    }
}
