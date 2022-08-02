--! insert(user_id, vault_id, ecdh_public_key, encrypted_vault_key)
INSERT INTO 
    users_vaults (user_id, vault_id, ecdh_public_key, encrypted_vault_key)
VALUES(:user_id, :vault_id, :ecdh_public_key, :encrypted_vault_key);

--! delete(vault_id, user_id, current_user_id)
DELETE FROM
    users_vaults
WHERE
    vault_id = :vault_id
AND
    user_id = :user_id
AND vault_id IN (SELECT vault_id FROM users_vaults WHERE user_id = :current_user_id);

--! get(user_id, vault_id)
SELECT 
    vault_id, user_id, encrypted_vault_key, ecdh_public_key 
FROM users_vaults 
WHERE 
    user_id = :user_id AND vault_id = :vault_id;

--! get_users_dangerous(vault_id) : (environments?)
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
    uv.vault_id = :vault_id;

-- Fetch members of the team that have not been added to this vault
--! get_non_members_dangerous(organisation_id, vault_id)
SELECT 
    u.id, 
    u.email,
    u.ecdh_public_key
FROM users u
WHERE 
    u.id IN (SELECT user_id FROM organisation_users WHERE organisation_id = :organisation_id)
AND
    u.id NOT IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);

--! remove_user_from_vault(vault_id, user_id, current_user)
DELETE FROM
    users_vaults
WHERE
    vault_id = :vault_id
AND
    user_id = :user_id
AND vault_id IN (SELECT vault_id FROM users_vaults WHERE user_id = :current_user);