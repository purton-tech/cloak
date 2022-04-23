--! insert(user_id, vault_id, ecdh_public_key, encrypted_vault_key)
INSERT INTO 
    users_vaults (user_id, vault_id, ecdh_public_key, encrypted_vault_key)
VALUES($1, $2, $3, $4) 

--! delete(vault_id, user_id, current_user_id)
DELETE FROM
    users_vaults
WHERE
    vault_id = $1
AND
    user_id =$2
AND vault_id IN (SELECT vault_id FROM users_vaults WHERE user_id = $3)

--! get(user_id, vault_id) { vault_id, user_id, encrypted_vault_key, ecdh_public_key } ?
SELECT 
    vault_id, user_id, encrypted_vault_key, ecdh_public_key 
FROM users_vaults 
WHERE 
    user_id = $1 AND vault_id = $2

--! get_users_dangerous(vault_id) { vault_id, user_id, encrypted_vault_key, ecdh_public_key } *
SELECT 
    uv.vault_id, uv.user_id, u.email  
FROM users_vaults uv
LEFT JOIN users u ON u.id = uv.user_id
WHERE 
    uv.vault_id = $1