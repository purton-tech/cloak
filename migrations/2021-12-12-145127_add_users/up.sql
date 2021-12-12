CREATE USER application LOGIN ENCRYPTED PASSWORD 'testpassword';
CREATE USER authentication LOGIN ENCRYPTED PASSWORD 'testpassword';
CREATE USER readonly LOGIN ENCRYPTED PASSWORD 'testpassword';

-- Allow our user to run SELECT, INSERT, UPDATE, DELETE queries.
GRANT SELECT, INSERT, UPDATE, DELETE
    ON ALL TABLES
    IN SCHEMA public
    TO application;
-- Enable this for all new tables.
ALTER DEFAULT PRIVILEGES
    GRANT SELECT, INSERT, UPDATE, DELETE
    ON TABLES
    TO application;
-- Allow our user to use SEQUENCES.
-- It's required to insert data with auto-incrementing primary keys for instance.
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO application;

ALTER DEFAULT PRIVILEGES
    GRANT USAGE, SELECT
    ON SEQUENCES
    TO application;

-- A readonly user
GRANT SELECT ON ALL TABLES IN SCHEMA public TO readonly;

-- Authnetication user. Only needs to see users and sessions
GRANT SELECT, INSERT, UPDATE, DELETE
    ON users
    TO authentication;

-- Authnetication user. Only needs to see users and sessions
GRANT SELECT, INSERT, UPDATE, DELETE
    ON sessions
    TO authentication;
    
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO authentication;