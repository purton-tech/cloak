CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    master_password_hash VARCHAR NOT NULL, 
    protected_symmetric_key VARCHAR NOT NULL, 
    protected_ecdsa_private_key VARCHAR NOT NULL, 
    ecdsa_public_key VARCHAR NOT NULL, 
    protected_ecdh_private_key VARCHAR NOT NULL, 
    ecdh_public_key VARCHAR NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY, 
    session_verifier VARCHAR NOT NULL, 
    user_id INT NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    otp_code_encrypted VARCHAR NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    otp_code_sent BOOLEAN NOT NULL DEFAULT false
);

-- Manage the updated_at column
SELECT updated_at('users');

-- Allow our user to run SELECT, INSERT, UPDATE, DELETE queries.
GRANT SELECT, INSERT, UPDATE, DELETE
    ON ALL TABLES
    IN SCHEMA public
    TO keyvault_app;
-- Enable this for all new tables.
ALTER DEFAULT PRIVILEGES
    GRANT SELECT, INSERT, UPDATE, DELETE
    ON TABLES
    TO keyvault_app;
-- Allow our user to use SEQUENCES.
-- It's required to insert data with auto-incrementing primary keys for instance.
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO keyvault_app;

ALTER DEFAULT PRIVILEGES
    GRANT USAGE, SELECT
    ON SEQUENCES
    TO keyvault_app;

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