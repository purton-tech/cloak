--! permissions(current_user_id, organisation_id)
SELECT 
    permission
FROM 
    roles_permissions
WHERE 
    role
IN 
    (
        SELECT UNNEST(roles) 
        FROM organisation_users 
        WHERE user_id = :current_user_id AND organisation_id = :organisation_id);