--! get_all_dangerous
SELECT  
    id, service_account_id, name, name_blind_index, secret, ecdh_public_key 
FROM 
    service_account_secrets 
WHERE 
    service_account_id = :service_account_id;

--! get_users_vaults
SELECT user_id 
FROM 
    users_vaults 
WHERE 
    user_id = :user_id
AND
    vault_id = :vault_id;

--! insert
INSERT INTO service_account_secrets
    (service_account_id, name, name_blind_index, secret, ecdh_public_key)
VALUES
    (
        :service_account_id, 
        :name, 
        :name_blind_index, 
        :secret, 
        :ecdh_public_key
);