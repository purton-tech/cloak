-- These tables are used by barricade to manage authentication

-- migrate:up
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    first_name VARCHAR, 
    last_name VARCHAR, 
    master_password_hash VARCHAR NOT NULL, 
    protected_symmetric_key VARCHAR NOT NULL, 
    protected_ecdsa_private_key VARCHAR NOT NULL, 
    ecdsa_public_key VARCHAR NOT NULL, 
    protected_ecdh_private_key VARCHAR NOT NULL, 
    ecdh_public_key VARCHAR NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE users IS 'Contains users and their private and public keys';
COMMENT ON COLUMN users.first_name IS 'The first name, not captured on registration for faster on boarding.';
COMMENT ON COLUMN users.last_name IS 'The last name, not captured on registration for faster on boarding.';
COMMENT ON COLUMN users.master_password_hash IS 'Hash of the users master password for authentication';
COMMENT ON COLUMN users.protected_symmetric_key IS 'Wrapped AES-GCM key for symmetric encryption and decryption';
COMMENT ON COLUMN users.protected_ecdsa_private_key IS 'Wrapped ECDSA key for signing';
COMMENT ON COLUMN users.ecdsa_public_key IS 'Public ECDSA key for signature verification';
COMMENT ON COLUMN users.protected_ecdh_private_key IS 'Wrapped ECDH key for public key encryption and key negotiation';
COMMENT ON COLUMN users.ecdh_public_key IS 'Public ECDH key for public key encryption and key negotiation';

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

COMMENT ON TABLE sessions IS 'The users login sessions';
COMMENT ON COLUMN sessions.session_verifier IS ' The session is a 32 byte random number stored in their cookie. This is the sha256 hash of that value.';
COMMENT ON COLUMN sessions.otp_code_encrypted IS 'A 6 digit code that is encrypted here to prevent attackers with read access to the database being able to use it.';
COMMENT ON COLUMN sessions.otp_code_attempts IS 'We count OTP attempts to prevent brute forcing.';
COMMENT ON COLUMN sessions.otp_code_confirmed IS 'Once the user enters the correct value this gets set to true.';
COMMENT ON COLUMN sessions.otp_code_sent IS 'Have we sent the OTP code?';

-- Give access to the application user, the application user has no access to 
-- The sessions table and therefore cannot fake a login.
GRANT SELECT, UPDATE ON users TO application;
GRANT SELECT ON users_id_seq TO application;

-- Give access to the readonly user
GRANT SELECT ON sessions, users, sessions_id_seq TO readonly;

-- Give access to authentication user
GRANT SELECT, INSERT, UPDATE, DELETE ON sessions TO authentication;
GRANT USAGE, SELECT ON sessions_id_seq TO authentication;
GRANT SELECT, INSERT, UPDATE ON users TO authentication;
GRANT USAGE, SELECT ON users_id_seq TO authentication;

-- Manage the updated_at column
SELECT updated_at('users');

-- migrate:down
DROP TABLE users;
DROP TABLE sessions;