--! audit(id?, action?, access_type?, user_id?) : Audit()
SELECT 
    id,
    (SELECT email from users WHERE id = user_id) as email,
    created_at,
    action, 
    access_type, 
    description 
FROM 
    audit_trail
WHERE 
    -- The inputs are optional in which case we can use COALESCE to skip
    id < COALESCE(:id, 2147483647)
    AND action = COALESCE(:action, action)
    AND access_type = COALESCE(:access_type, access_type)
    AND user_id = COALESCE(:user_id, user_id)
    AND organisation_id = :organisation_id
ORDER BY created_at DESC
LIMIT :limit;

--! insert
INSERT INTO 
    audit_trail (user_id, organisation_id, action, access_type, description) 
    VALUES (:user_id, :organisation_id, :action, :access_type, :description);