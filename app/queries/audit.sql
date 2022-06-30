--! audit(current_user_id) { email, created_at, action,  access_type, description } *
SELECT 
    (SELECT email from users WHERE id = user_id) as email,
    created_at,
    action, 
    access_type, 
    description 
FROM audit_trail
WHERE user_id IN
    -- Make sure the current user is an admin for this team
    (SELECT 
        user_id 
    FROM 
        organisation_users 
    WHERE
        organisation_id IN (SELECT 
                id
            FROM 
                organisations
            WHERE
                created_by_user_id = $1
        )
    )
ORDER BY created_at DESC
LIMIT 50;

--! insert(user_id, action, access_type, description)
INSERT INTO audit_trail (user_id, action, access_type, description) VALUES ($1, $2, $3, $4)