DELETE FROM service_account_secrets;
ALTER TABLE service_account_secrets ADD COLUMN name_blind_index VARCHAR NOT NULL;