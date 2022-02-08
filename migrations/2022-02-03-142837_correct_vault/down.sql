DELETE FROM users_vaults;
DELETE FROM vaults;
DELETE FROM service_account_secrets;

ALTER TABLE vaults ADD COLUMN encrypted_ecdh_private_key VARCHAR NOT NULL;
ALTER TABLE vaults ADD COLUMN ecdh_public_key VARCHAR NOT NULL;

ALTER TABLE users_vaults DROP COLUMN ecdh_public_key;
ALTER TABLE service_account_secrets DROP COLUMN ecdh_public_key;