--! insert(vault_id, name, name_blind_index, secret)
INSERT INTO 
    secrets (vault_id, name, name_blind_index, secret)
VALUES($1, $2, $3, $4) 

--! get_all(vault_id, current_user_id) { id, vault_id, name, name_blind_index, secret, updated_at, created_at } *
SELECT  
    id, vault_id, name, name_blind_index, secret,
    updated_at, created_at  
FROM secrets WHERE vault_id = $1
AND
    vault_id 
IN
    (SELECT vault_id 
    FROM
        users_vaults
    WHERE
        user_id = $2)

--! get(id, current_user_id) { id, vault_id, name, name_blind_index, secret, updated_at, created_at } ?
SELECT  
    id, vault_id, name, name_blind_index, secret,
    updated_at, created_at  
FROM secrets WHERE id = $1
AND
    vault_id 
IN
    (SELECT vault_id 
    FROM
        users_vaults
    WHERE
        user_id = $2)

--! delete_secrets(id, current_user_id)
DELETE FROM
    secrets
WHERE
    id = $1
AND
    vault_id 
IN
    (SELECT vault_id 
    FROM
        users_vaults
    WHERE
        user_id = $2)

--! delete_service_account(name_blind_index, vault_id)
DELETE FROM
    service_account_secrets
WHERE
    name_blind_index = $1
AND
    service_account_id
IN
    (SELECT id FROM service_accounts WHERE vault_id = $2)