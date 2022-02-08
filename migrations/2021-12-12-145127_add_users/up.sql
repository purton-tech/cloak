DO $$
BEGIN
  CREATE ROLE cloak LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role cloak -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE cloak_auth LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role cloak_auth -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE cloak_readonly LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role cloak_readonly -- it already exists';
END
$$;

-- Allow our user to run SELECT, INSERT, UPDATE, DELETE queries.
GRANT SELECT, INSERT, UPDATE, DELETE
    ON ALL TABLES
    IN SCHEMA public
    TO cloak;
-- Allow our user to use SEQUENCES.
-- It's required to insert data with auto-incrementing primary keys for instance.
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO cloak;

ALTER DEFAULT PRIVILEGES
    GRANT USAGE, SELECT
    ON SEQUENCES
    TO cloak;

-- A readonly user
GRANT SELECT ON ALL TABLES IN SCHEMA public TO cloak_readonly;

-- Authnetication user. Only needs to see users and sessions
GRANT SELECT, INSERT, UPDATE, DELETE
    ON users
    TO cloak_auth;

-- Authnetication user. Only needs to see users and sessions
GRANT SELECT, INSERT, UPDATE, DELETE
    ON sessions
    TO cloak_auth;
    
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO cloak_auth;