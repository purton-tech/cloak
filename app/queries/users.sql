--! get_dangerous(id) { id, email, ecdsa_public_key } ?
SELECT 
    id, email, ecdsa_public_key
FROM 
    users
WHERE
    id = $1

--! get_by_email_dangerous(email) { id, email, ecdsa_public_key } ?
SELECT 
    id, email, ecdsa_public_key
FROM 
    users
WHERE
    email = $1
