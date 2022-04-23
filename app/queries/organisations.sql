--! get_organisations(created_by_user_id) { id, name } * 
SELECT 
    id, name
FROM 
    organisations
WHERE
    created_by_user_id = $1

--! add_user_to_organisation(user_id, organisation_id)
INSERT INTO 
    organisation_users (user_id, organisation_id)
VALUES($1, $2) 

--! insert_organisation(created_by_user_id) 
INSERT INTO 
    organisations (created_by_user_id)
VALUES($1) 
RETURNING id

--! insert_user_into_org(user_id, organisation_id, is_admin)
INSERT INTO 
    organisation_users (user_id, organisation_id, is_admin)
VALUES($1, $2, $3) 

--! get_users(user_id, organisation_id) { id, organisation_id, email, ecdh_public_key, is_admin} *
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

--! get_teams(user_id) { organisation_name, team_owner } *
SELECT 
    o.name as organisation_name, 
    u.email as team_owner
FROM 
    organisation_users ou
LEFT JOIN organisations o ON o.id = ou.organisation_id
LEFT JOIN users u ON u.id = o.created_by_user_id
WHERE
    ou.user_id = $1

--! remove_user(user_id_to_remove, organisation_id, current_user)
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