--! permissions(current_user_id, organisation_id) { permission } *
SELECT 
    permission
FROM 
    roles_permissions
WHERE 
    role
IN 
    (SELECT UNNEST(roles) FROM organisation_users WHERE user_id = $1 AND organisation_id = $2)