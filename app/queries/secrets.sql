--! insert(vault_id, name, name_blind_index, secret, environment_id)
INSERT INTO 
    secrets (vault_id, name, name_blind_index, secret, environment_id)
VALUES($1, $2, $3, $4, $5) 

--! get_all(vault_id) 
--!     { id, vault_id, name, name_blind_index, secret, 
--!     environment_id, environment_name, updated_at, created_at } *
SELECT  
    id, 
    vault_id, 
    name, 
    name_blind_index, 
    secret, 
    environment_id,
    (SELECT name from environments WHERE id = environment_id) AS environment_name,
    updated_at, 
    created_at  
FROM secrets WHERE vault_id = $1
ORDER BY environment_name

--! get(id) { id, vault_id, name, name_blind_index, secret, updated_at, created_at }
SELECT  
    id, vault_id, name, name_blind_index, secret,
    updated_at, created_at  
FROM secrets WHERE id = $1

--! delete_secret(id)
DELETE FROM
    secrets
WHERE
    id = $1

--! delete_service_account(name_blind_index, vault_id)
DELETE FROM
    service_account_secrets
WHERE
    name_blind_index = $1
AND
    service_account_id
IN
    (SELECT id FROM service_accounts WHERE vault_id = $2)