DELETE FROM users_vaults;
DELETE FROM vaults;

ALTER TABLE vaults DROP COLUMN encrypted_ecdh_private_key;
ALTER TABLE vaults DROP COLUMN ecdh_public_key;

ALTER TABLE users_vaults ADD COLUMN ecdh_public_key VARCHAR NOT NULL;

ALTER TABLE service_account_secrets ADD COLUMN ecdh_public_key VARCHAR NOT NULL;