--! audit() { email, created_at, action,  access_type, description } *
SELECT 
    (SELECT email from users WHERE id = user_id) as email,
    created_at,
    action, 
    access_type, 
    description 
FROM audit_trail
ORDER BY created_at DESC
LIMIT 50;

--! insert(user_id, action, access_type, description)
INSERT INTO audit_trail (user_id, action, access_type, description) VALUES ($1, $2, $3, $4)