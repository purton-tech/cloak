--! connect(vault_id, id, current_user_id)
 UPDATE service_accounts 
SET 
    vault_id = $1
WHERE 
    id = $2
AND 
    -- Make sure the user has access to the vault
    $1 IN (SELECT vault_id from users_vaults WHERE user_id = $3)
AND user_id = $3

--! insert(user_id, name, ecdh_public_key, encrypted_ecdh_private_key)
INSERT INTO 
    service_accounts (user_id, name, ecdh_public_key, encrypted_ecdh_private_key)
VALUES($1, $2, $3, $4) 

--! get_all(user_id) { id, vault_id?, account_name, vault_name?, ecdh_public_key, encrypted_ecdh_private_key, updated_at, created_at } *
SELECT 
    sa.id, sa.vault_id, sa.name, v.name as vault_name, 
    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
    sa.updated_at, sa.created_at 
FROM 
    service_accounts sa
LEFT JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE 
    sa.user_id = $1

--! get_by_vault(vault_id, current_user_id) { id, vault_id?, account_name, vault_name?, ecdh_public_key, encrypted_ecdh_private_key, updated_at, created_at } *
SELECT 
    sa.id, sa.vault_id, sa.name, v.name as vault_name, 
    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
    sa.updated_at, sa.created_at 
FROM 
    service_accounts sa
LEFT OUTER JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE 
    sa.vault_id = $1
    -- Make sure the user actually as access to this vault
    AND
        $2 IN
            (SELECT user_id 
            FROM
                users_vaults
            WHERE
                vault_id = $1)

--! get_by_ecdh_public_key(ecdh_public_key) { id, vault_id?, account_name, vault_name, ecdh_public_key, encrypted_ecdh_private_key, updated_at, created_at }
SELECT 
    sa.id, sa.vault_id, sa.name, v.name as vault_name, 
    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
    sa.updated_at, sa.created_at 
FROM 
    service_accounts sa
LEFT OUTER JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE sa.ecdh_public_key = $1

--! get_dangerous(id) { id, vault_id?, account_name, vault_name, ecdh_public_key, encrypted_ecdh_private_key, updated_at, created_at }
SELECT
    sa.id, sa.vault_id, sa.name, v.name as vault_name, 
    sa.ecdh_public_key, sa.encrypted_ecdh_private_key,
    sa.updated_at, sa.created_at 
FROM 
    service_accounts sa
LEFT OUTER JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE
    sa.id = $1

--! delete_service_account(id, current_user_id)
DELETE FROM
    service_accounts
WHERE
    id = $1
AND
    user_id = $2

--! delete_service_account_secrets(service_account_id)
DELETE FROM
    service_account_secrets
WHERE
    service_account_id = $1