--: Secret()

--! insert
INSERT INTO 
    secrets (vault_id, name, name_blind_index, secret, environment_id)
VALUES(
    :vault_id, 
    :name, 
    :name_blind_index, 
    :secret, 
    :environment_id
);

--! get_all : Secret
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
FROM secrets WHERE vault_id = :vault_id
ORDER BY environment_name;

--! get : Secret
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
FROM secrets WHERE id = :id;

--! delete_secret
DELETE FROM
    secrets
WHERE
    id = :id;

--! delete_service_account
DELETE FROM
    service_account_secrets
WHERE
    name_blind_index = :name_blind_index
AND
    service_account_id
IN
    (SELECT id FROM service_accounts WHERE vault_id = :vault_id);