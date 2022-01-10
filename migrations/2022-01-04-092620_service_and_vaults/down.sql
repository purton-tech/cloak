DROP TABLE service_accounts;
DROP TABLE service_account_secrets;

ALTER TABLE vaults DROP COLUMN encrypted_ecdh_private_key;
ALTER TABLE vaults DROP COLUMN ecdh_public_key;
