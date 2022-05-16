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

--! get(user_id, vault_id) { vault_id, user_id, encrypted_vault_key, ecdh_public_key }
SELECT 
    vault_id, user_id, encrypted_vault_key, ecdh_public_key 
FROM users_vaults 
WHERE 
    user_id = $1 AND vault_id = $2

--! get_users_dangerous(vault_id) { vault_id, user_id, email, environments? } *
SELECT 
    uv.vault_id, 
    uv.user_id, 
    u.email,
    -- Creata a string showing the users environments for this vault
    (
        SELECT 
            STRING_AGG(e.name, ', ') 
        FROM 
            users_environments ue
        LEFT JOIN environments e ON ue.environment_id = e.id 
        WHERE 
            ue.user_id = uv.user_id
        AND
            e.vault_id = uv.vault_id
    ) 
    as environments 
FROM users_vaults uv
LEFT JOIN users u ON u.id = uv.user_id
WHERE 
    uv.vault_id = $1

--! remove_user_from_vault(user_id, vault_id, current_user)
DELETE FROM
    users_vaults
WHERE
    vault_id = $1
AND
    user_id =$2
AND vault_id IN (SELECT vault_id FROM users_vaults WHERE user_id = $3)