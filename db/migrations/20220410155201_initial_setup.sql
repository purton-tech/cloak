-- migrate:up


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- These roles are only created in development. In production they will
-- have already been cfreated by the infrastructure as code and have secret passwords.

DO $$
BEGIN
  CREATE ROLE application LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role application -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE authentication LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role authentication -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE readonly LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role readonly -- it already exists';
END
$$;

-- migrate:down
DROP OWNED BY application;
DROP OWNED BY authentication;
DROP OWNED BY readonly;

DROP USER application;
DROP USER authentication;
DROP USER readonly;

