ALTER TABLE vaults ALTER updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE vaults ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC';

ALTER TABLE secrets ALTER updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE secrets ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC';

ALTER TABLE service_accounts ALTER updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE service_accounts ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC';

ALTER TABLE service_account_secrets ALTER updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE service_account_secrets ALTER created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC';