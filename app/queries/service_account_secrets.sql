--! get_all_dangerous(service_account_id) { id, service_account_id, name, name_blind_index, secret, ecdh_public_key } *
SELECT  
    id, service_account_id, name, name_blind_index, secret, ecdh_public_key 
FROM 
    service_account_secrets 
WHERE 
    service_account_id = $1

--! get_users_vaults(user_id, vault_id) *
SELECT user_id 
FROM 
    users_vaults 
WHERE 
    user_id = $1
AND
    vault_id = $2

--! insert(service_account_id, name, name_blind_index, secret, ecdh_public_key)
INSERT INTO service_account_secrets
    (service_account_id, name, name_blind_index, secret, ecdh_public_key)
VALUES
    ($1, $2, $3, $4, $5)