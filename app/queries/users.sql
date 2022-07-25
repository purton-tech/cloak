--! get_dangerous(id) { id, email, ecdsa_public_key }
SELECT 
    id, email, ecdsa_public_key
FROM 
    users
WHERE
    id = $1

--! get_by_email_dangerous(email) { id, email, ecdsa_public_key }
SELECT 
    id, email, ecdsa_public_key
FROM 
    users
WHERE
    email = $1

--! set_name(current_user_id, first_name, last_name)
UPDATE
    users
SET 
    first_name = $2, last_name = $3 
WHERE
    id = $1
