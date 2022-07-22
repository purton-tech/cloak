--! insert(organisation_id, name)
INSERT INTO 
    vaults (organisation_id, name)
VALUES($1, $2) 
RETURNING id

--! insert_user_vaults(user_id, vault_id, ecdh_public_key, encrypted_vault_key)
INSERT INTO 
    users_vaults (user_id, vault_id, ecdh_public_key, encrypted_vault_key)
VALUES($1, $2, $3, $4) 

--! get_dangerous(id) { id, name, updated_at, created_at }
SELECT 
    id, name, updated_at, created_at
FROM 
    vaults
WHERE
    id = $1 

--! get(id, current_user_id) { id, name, updated_at, created_at }
SELECT 
    id, name, updated_at, created_at
FROM 
    vaults
WHERE
    id = $1 
AND
    $1 
IN
    (SELECT vault_id 
    FROM
        users_vaults
    WHERE
        user_id = $2)

--! get_all(current_user_id, organisation_id) { id, name, updated_at, created_at } *
SELECT 
    v.id, v.name, v.updated_at, v.created_at
FROM 
    vaults v
LEFT JOIN users_vaults uv ON uv.vault_id = v.id
WHERE
    uv.user_id = $1
AND
    v.organisation_id = $2

--! user_vault_count(vault_id)
SELECT count(*) FROM users_vaults WHERE vault_id = $1

--! secrets_count(vault_id)
SELECT count(*) FROM secrets WHERE vault_id = $1

--! delete(vault_id, current_user_id)
DELETE FROM
    vaults
WHERE
    id = $1
AND
    $2 IN (SELECT user_id FROM users_vaults WHERE vault_id = $1)

--! remove_vault_from_service_accounts(vault_id, current_user_id)
UPDATE
    service_accounts
SET
    vault_id = NULL
WHERE
    vault_id = $1
AND
    $2 IN (SELECT user_id FROM users_vaults WHERE vault_id = $1)

--! delete_vault_secrets(vault_id, current_user_id)
DELETE FROM
    secrets
WHERE
    vault_id = $1
AND
    $2 IN (SELECT user_id FROM users_vaults WHERE vault_id = $1)