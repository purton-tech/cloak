-- migrate:up
ALTER TABLE secrets ADD COLUMN folder VARCHAR NOT NULL DEFAULT '/';

-- migrate:down
ALTER TABLE secrets DROP COLUMN folder;