--! insert(organisation_id, name)
INSERT INTO 
    vaults (organisation_id, name)
VALUES(:organisation_id, :name) 
RETURNING id;

--! insert_user_vaults(user_id, vault_id, ecdh_public_key, encrypted_vault_key)
INSERT INTO 
    users_vaults (user_id, vault_id, ecdh_public_key, encrypted_vault_key)
VALUES(
    :user_id, 
    :vault_id, 
    :ecdh_public_key, 
    :encrypted_vault_key
);

--! get_dangerous(id)
SELECT 
    id, name, updated_at, created_at
FROM 
    vaults
WHERE
    id = :id;

--! get(id, current_user_id)
SELECT 
    id, name, updated_at, created_at
FROM 
    vaults
WHERE
    id = :id 
AND
    :id 
IN
    (SELECT vault_id 
    FROM
        users_vaults
    WHERE
        user_id = :current_user_id);

--! get_all(current_user_id, organisation_id)
SELECT 
    v.id, v.name, v.updated_at, v.created_at
FROM 
    vaults v
LEFT JOIN users_vaults uv ON uv.vault_id = v.id
WHERE
    uv.user_id = :current_user_id
AND
    v.organisation_id = :organisation_id;

--! user_vault_count(vault_id)
SELECT count(*) FROM users_vaults WHERE vault_id = :vault_id;

--! secrets_count(vault_id)
SELECT count(*) FROM secrets WHERE vault_id = :vault_id;

--! delete(vault_id, current_user_id)
DELETE FROM
    vaults
WHERE
    id = :vault_id
AND
    :current_user_id IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);

--! remove_vault_from_service_accounts(vault_id, current_user_id)
UPDATE
    service_accounts
SET
    vault_id = NULL
WHERE
    vault_id = :vault_id
AND
    :current_user_id IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);

--! delete_vault_secrets(vault_id, current_user_id)
DELETE FROM
    secrets
WHERE
    vault_id = :vault_id
AND
    :current_user_id IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);