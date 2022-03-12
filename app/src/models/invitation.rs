use crate::authentication::Authentication;
use crate::errors::CustomError;
use crate::models::organisation;
use sqlx::PgPool;

pub struct Invitation {
    pub id: i32,
    pub organisation_id: i32,
    pub email: String,
    pub invitation: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Invitation {
    pub async fn create(
        pool: &PgPool,
        authenticated_user: &Authentication,
        organisation_id: u32,
        email: String,
        invitation: String,
    ) -> Result<(), CustomError> {
        if let Ok(_org_user) = organisation::Organisation::get_dangerous(
            pool,
            authenticated_user.user_id,
            organisation_id,
        )
        .await
        {
            sqlx::query!(
                "
                    INSERT INTO 
                        invitations (organisation_id, email, invitation)
                    VALUES($1, $2, $3) 
                ",
                organisation_id as i32,
                email,
                invitation,
            )
            .execute(pool)
            .await?;

            return Ok(());
        }

        Err(CustomError::Unauthorized(
            "suspicious idor request".to_string(),
        ))
    }

    pub async fn delete_dangerous(
        pool: &PgPool,
        email: &str,
        organisation_id: u32,
    ) -> Result<(), CustomError> {
        sqlx::query!(
            r#"
                DELETE FROM
                    invitations
                WHERE
                    email = $1
                AND
                    organisation_id = $2
            "#,
            email,
            organisation_id as i32
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(
        pool: &PgPool,
        authenticated_user: &Authentication,
        organisation_id: u32,
    ) -> Result<Vec<Invitation>, CustomError> {
        Ok(sqlx::query_as!(
            Invitation,
            "
                SELECT  
                    id, 
                    email,
                    invitation,
                    organisation_id,
                    updated_at, 
                    created_at  
                FROM 
                    invitations 
                WHERE organisation_id = $1
                    AND
                        organisation_id 
                    IN
                        (SELECT id 
                        FROM
                            organisation_users
                        WHERE
                            user_id = $2)
            ",
            organisation_id as i32,
            authenticated_user.user_id as i32
        )
        .fetch_all(pool)
        .await?)
    }
}
