use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct Organisation {
    pub id: i32,
    pub name: Option<String>,
}

pub struct User {
    pub id: i32,
    pub email: String,
    pub ecdh_public_key: String,
    pub is_admin: bool,
}

impl Organisation {
    pub async fn get_primary_org(
        pool: &PgPool,
        authenticated_user: &Authentication,
    ) -> Result<Organisation, CustomError> {
        Ok(sqlx::query_as!(
            Organisation,
            "
                SELECT 
                    id, name
                FROM 
                    organisations
                WHERE
                    created_by_user_id = $1
            ",
            authenticated_user.user_id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    // Add an invited user to the organisation
    pub async fn add_user_dangerous(
        pool: &PgPool,
        email: &str,
        organisation_id: u32,
    ) -> Result<(), CustomError> {
        let user = super::user::User::get_by_email_dangerous(pool, email).await?;

        sqlx::query!(
            "
                INSERT INTO 
                    organisation_users (user_id, organisation_id)
                VALUES($1, $2) 
            ",
            user.id,
            organisation_id as i32,
        )
        .execute(pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn create(
        pool: &PgPool,
        authenticated_user: &Authentication,
    ) -> Result<(), CustomError> {
        let result = sqlx::query!(
            "
                INSERT INTO 
                    organisations (created_by_user_id)
                VALUES($1) 
                RETURNING ID
            ",
            authenticated_user.user_id as i32
        )
        .fetch_one(pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        sqlx::query!(
            "
                INSERT INTO 
                    organisation_users (user_id, organisation_id, is_admin)
                VALUES($1, $2, $3) 
            ",
            authenticated_user.user_id as i32,
            result.id,
            true
        )
        .execute(pool)
        .await
        .map_err(|e| CustomError::Database(e.to_string()))?;

        Ok(())
    }

    pub async fn get_users(
        pool: &PgPool,
        authenticated_user: &Authentication,
        idor_organisation_id: i32,
    ) -> Result<Vec<User>, CustomError> {
        Ok(sqlx::query_as!(
            User,
            "
                SELECT 
                    u.id, u.email, u.ecdh_public_key, ou.is_admin
                FROM 
                    organisation_users ou
                LEFT JOIN users u ON u.id = ou.user_id
                WHERE
                    ou.organisation_id = $2
                AND
                    -- Make sure the user has access to this org
                    $1 IN (SELECT user_id FROM organisation_users WHERE organisation_id = $2)
            ",
            authenticated_user.user_id as i32,
            idor_organisation_id
        )
        .fetch_all(pool)
        .await?)
    }
}
