--! audit(current_user_id, organisation_id) { email, created_at, action,  access_type, description } *
SELECT 
    (SELECT email from users WHERE id = user_id) as email,
    created_at,
    action, 
    access_type, 
    description 
FROM audit_trail
WHERE organisation_id IN
    -- Bring back the audit items this user has access to.
    (SELECT 
        organisation_id 
    FROM 
        organisation_users 
    WHERE
        organisation_id = $2 and user_id = $1
    )
ORDER BY created_at DESC
LIMIT 50;

--! insert(user_id, organisation_id, action, access_type, description)
INSERT INTO audit_trail (user_id, organisation_id, action, access_type, description) VALUES ($1, $2, $3, $4, $5)