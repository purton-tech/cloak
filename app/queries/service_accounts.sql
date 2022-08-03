--! connect
UPDATE service_accounts 
SET 
    vault_id = :vault_id, environment_id = :environment_id
WHERE 
    id = :id
AND 
    -- Make sure the user has access to the vault
    :vault_id IN (SELECT vault_id from users_vaults WHERE user_id = :current_user_id)
AND organisation_id = :organisation_id;

--! insert
INSERT INTO 
    service_accounts (organisation_id, name, ecdh_public_key, encrypted_ecdh_private_key)
VALUES(
    :organisation_id, 
    :name, 
    :ecdh_public_key, 
    :encrypted_ecdh_private_key
);

--! get_all : (vault_id?, vault_name?, environment_name?)
SELECT 
    sa.id, 
    sa.vault_id, 
    sa.name as account_name,
    (SELECT name FROM vaults WHERE id = sa.vault_id) as vault_name,
    (SELECT name FROM environments WHERE id = sa.environment_id) as environment_name,
    sa.ecdh_public_key, 
    sa.encrypted_ecdh_private_key,
    sa.updated_at, 
    sa.created_at 
FROM 
    service_accounts sa
WHERE 
    sa.organisation_id = :organisation_id;

--! get_by_vault : (vault_id?, environment_id?)
SELECT 
    sa.id as id, 
    sa.vault_id as vault_id, 
    sa.name as name, 
    v.name as vault_name, 
    sa.ecdh_public_key, 
    sa.encrypted_ecdh_private_key,
    sa.environment_id,
    sa.updated_at, 
    sa.created_at 
FROM 
    service_accounts sa
LEFT OUTER JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE 
    sa.vault_id = :vault_id;

--! get_by_ecdh_public_key : (vault_id?, vault_name?)
SELECT 
    sa.id, 
    sa.vault_id, 
    sa.name as account_name, 
    v.name as vault_name, 
    sa.ecdh_public_key, 
    sa.encrypted_ecdh_private_key,
    sa.updated_at, 
    sa.created_at 
FROM 
    service_accounts sa
LEFT OUTER JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE sa.ecdh_public_key = :ecdh_public_key;

--! get_dangerous : (vault_id?, vault_name?)
SELECT
    sa.id, 
    sa.vault_id, 
    sa.name, 
    v.name as vault_name, 
    sa.ecdh_public_key,
    sa.encrypted_ecdh_private_key,
    sa.updated_at, 
    sa.created_at 
FROM 
    service_accounts sa
LEFT OUTER JOIN
    vaults v
ON 
    v.id = sa.vault_id
WHERE
    sa.id = :id;

--! delete_service_account
DELETE FROM
    service_accounts
WHERE
    id = :id
AND
    organisation_id = :organisation_id;

--! delete_service_account_secrets
DELETE FROM
    service_account_secrets
WHERE
    service_account_id = :service_account_id;