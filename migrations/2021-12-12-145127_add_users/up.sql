DO $$
BEGIN
  CREATE ROLE keyvault LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role keyvault -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE keyvault_auth LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role keyvault_auth -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE keyvault_readonly LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role keyvault_readonly -- it already exists';
END
$$;

-- Allow our user to run SELECT, INSERT, UPDATE, DELETE queries.
GRANT SELECT, INSERT, UPDATE, DELETE
    ON ALL TABLES
    IN SCHEMA public
    TO keyvault;
-- Enable this for all new tables.
ALTER DEFAULT PRIVILEGES
    GRANT SELECT, INSERT, UPDATE, DELETE
    ON TABLES
    TO keyvault;
-- Allow our user to use SEQUENCES.
-- It's required to insert data with auto-incrementing primary keys for instance.
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO keyvault;

ALTER DEFAULT PRIVILEGES
    GRANT USAGE, SELECT
    ON SEQUENCES
    TO keyvault;

-- A readonly user
GRANT SELECT ON ALL TABLES IN SCHEMA public TO keyvault_readonly;

-- Authnetication user. Only needs to see users and sessions
GRANT SELECT, INSERT, UPDATE, DELETE
    ON users
    TO keyvault_auth;

-- Authnetication user. Only needs to see users and sessions
GRANT SELECT, INSERT, UPDATE, DELETE
    ON sessions
    TO keyvault_auth;
    
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO keyvault_auth;