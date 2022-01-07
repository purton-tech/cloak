CREATE TABLE service_accounts (
    id SERIAL PRIMARY KEY, 
    user_id INT NOT NULL, 
    vault_id INT, 
    name VARCHAR NOT NULL,
    encrypted_ecdh_private_key VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE service_account_secrets (
    id SERIAL PRIMARY KEY, 
    service_account_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

DELETE FROM secrets;
DELETE FROM vaults;
DELETE FROM service_accounts;
ALTER TABLE vaults ADD COLUMN encrypted_ecdh_private_key VARCHAR NOT NULL;
ALTER TABLE vaults ADD COLUMN ecdh_public_key VARCHAR NOT NULL;

-- Manage the updated_at column
SELECT updated_at('service_accounts');