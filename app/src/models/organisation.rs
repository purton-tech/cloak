use crate::authentication::Authentication;
use crate::errors::CustomError;
use sqlx::PgPool;

pub struct Organisation {
    pub id: i32,
    pub name: Option<String>,
}

pub struct User {
    pub id: i32,
    pub organisation_id: i32,
    pub email: String,
    pub ecdh_public_key: String,
    pub is_admin: bool,
}

pub struct Team {
    pub team_owner: String,
    pub organisation_name: Option<String>,
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
                    u.id, ou.organisation_id, u.email, u.ecdh_public_key, ou.is_admin
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

    pub async fn get_teams(
        pool: &PgPool,
        current_user: &Authentication,
    ) -> Result<Vec<Team>, CustomError> {
        Ok(sqlx::query_as!(
            Team,
            "
                SELECT 
                    o.name as organisation_name, 
                    u.email as team_owner
                FROM 
                    organisation_users ou
                LEFT JOIN organisations o ON o.id = ou.organisation_id
                LEFT JOIN users u ON u.id = o.created_by_user_id
                WHERE
                    ou.user_id = $1
            ",
            current_user.user_id as i32,
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn remove_user(
        pool: &PgPool,
        current_user: &Authentication,
        organisation_id: u32,
        user_id: u32,
    ) -> Result<(), CustomError> {
        sqlx::query!(
            r#"
                DELETE FROM
                    organisation_users
                WHERE
                    user_id = $1
                AND
                    organisation_id = $2 
                and $3 IN
                -- Make sure the current user is an admin for this team
                    (SELECT 
                        user_id 
                    FROM 
                        organisation_users 
                    WHERE
                        is_admin = true
                    AND
                    organisation_id = $2
                    )
            "#,
            user_id as i32,
            organisation_id as i32,
            current_user.user_id as i32,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
