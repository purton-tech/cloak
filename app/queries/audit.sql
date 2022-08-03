--! audit
SELECT 
    (SELECT email from users WHERE id = user_id) as email,
    created_at,
    action, 
    access_type, 
    description 
FROM audit_trail
WHERE organisation_id = :organisation_id
ORDER BY created_at DESC
LIMIT 50;

--! insert
INSERT INTO 
    audit_trail (user_id, organisation_id, action, access_type, description) 
    VALUES (:user_id, :organisation_id, :action, :access_type, :description);