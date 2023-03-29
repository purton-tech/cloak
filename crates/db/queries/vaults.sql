--: Vault()

--! insert
INSERT INTO 
    vaults (organisation_id, name)
VALUES(:organisation_id, :name) 
RETURNING id;

--! insert_user_vaults
INSERT INTO 
    users_vaults (user_id, vault_id, ecdh_public_key, encrypted_vault_key)
VALUES(
    :user_id, 
    :vault_id, 
    :ecdh_public_key, 
    :encrypted_vault_key
);

--! vault : Vault
SELECT 
    id, 
    name, 
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(updated_at)::text) as updated_at, 
    trim(both '"' from to_json(created_at)::text) as created_at
FROM 
    vaults
WHERE
    id = :id;

--! get : Vault
SELECT 
    id, 
    name, 
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(updated_at)::text) as updated_at, 
    trim(both '"' from to_json(created_at)::text) as created_at
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

--! get_all : Vault
SELECT 
    v.id,
    v.name, 
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(v.updated_at)::text) as updated_at, 
    trim(both '"' from to_json(v.created_at)::text) as created_at
FROM 
    vaults v
LEFT JOIN users_vaults uv ON uv.vault_id = v.id
WHERE
    uv.user_id = :current_user_id
AND
    v.organisation_id = :organisation_id;

--! user_vault_count
SELECT count(*) FROM users_vaults WHERE vault_id = :vault_id;

--! secrets_count
SELECT count(*) FROM secrets WHERE vault_id = :vault_id;

--! delete
DELETE FROM
    vaults
WHERE
    id = :vault_id
AND
    :current_user_id IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);

--! rename
UPDATE
    vaults
SET 
    name = :name
WHERE
    id = :vault_id;

--! remove_vault_from_service_accounts
UPDATE
    service_accounts
SET
    vault_id = NULL
WHERE
    vault_id = :vault_id
AND
    :current_user_id IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);

--! delete_vault_secrets
DELETE FROM
    secrets
WHERE
    vault_id = :vault_id
AND
    :current_user_id IN (SELECT user_id FROM users_vaults WHERE vault_id = :vault_id);