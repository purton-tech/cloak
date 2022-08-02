--! organisation(org_id) : (name?)
SELECT 
    id, name
FROM 
    organisations
WHERE
    id = :org_id;

--! set_name(name, org_id)
UPDATE
    organisations
SET 
    name = :name
WHERE
    id = :org_id;

--! get_primary_organisation(created_by_user_id): (name?)
SELECT 
    id, name
FROM 
    organisations
WHERE
    created_by_user_id = :created_by_user_id;

--! add_user_to_organisation(user_id, organisation_id, roles)
INSERT INTO 
    organisation_users (user_id, organisation_id, roles)
VALUES(:user_id, :organisation_id, :roles);

--! insert_organisation
INSERT INTO 
    organisations (created_by_user_id)
VALUES(current_app_user()) 
RETURNING id;

--! get_users(organisation_id)
SELECT 
    u.id, ou.organisation_id, u.email, u.ecdh_public_key, ou.roles
FROM 
    organisation_users ou
LEFT JOIN users u ON u.id = ou.user_id
WHERE
    ou.organisation_id = :organisation_id;

--! get_teams(user_id) : (organisation_name?)
SELECT 
    o.id,
    o.name as organisation_name, 
    u.email as team_owner
FROM 
    organisation_users ou
LEFT JOIN organisations o ON o.id = ou.organisation_id
LEFT JOIN users u ON u.id = o.created_by_user_id
WHERE
    ou.user_id = :user_id
ORDER BY o.name ASC;

--! remove_user(user_id_to_remove, organisation_id)
DELETE FROM
    organisation_users
WHERE
    user_id = :user_id_to_remove
AND
    organisation_id = :organisation_id;