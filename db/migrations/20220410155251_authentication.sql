-- These tables are used by barricade to manage authentication

-- migrate:up
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    master_password_hash VARCHAR NOT NULL, 
    protected_symmetric_key VARCHAR NOT NULL, 
    protected_ecdsa_private_key VARCHAR NOT NULL, 
    ecdsa_public_key VARCHAR NOT NULL, 
    protected_ecdh_private_key VARCHAR NOT NULL, 
    ecdh_public_key VARCHAR NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY, 
    session_verifier VARCHAR NOT NULL, 
    user_id INT NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otp_code_encrypted VARCHAR NOT NULL,
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    otp_code_sent BOOLEAN NOT NULL DEFAULT false
);


-- Give access to the application user, the application user has no access to 
-- The sessions table and therefore cannot fake a login.
GRANT SELECT, INSERT, UPDATE, DELETE ON users TO application;
GRANT USAGE, SELECT ON users_id_seq TO application;

-- Give access to the readonly user
GRANT SELECT ON sessions TO readonly;
GRANT SELECT ON sessions_id_seq TO readonly;
GRANT SELECT ON users TO readonly;
GRANT SELECT ON users_id_seq TO readonly;

-- Give access to authentication user
GRANT SELECT, INSERT, UPDATE, DELETE ON sessions TO authentication;
GRANT USAGE, SELECT ON sessions_id_seq TO authentication;
GRANT SELECT, INSERT, UPDATE, DELETE ON users TO authentication;
GRANT USAGE, SELECT ON users_id_seq TO authentication;

-- Manage the updated_at column
SELECT updated_at('users');

-- migrate:down
DROP TABLE users;
DROP TABLE sessions;