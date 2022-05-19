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

CREATE TABLE vaults (
    id SERIAL PRIMARY KEY, 
    user_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE secrets (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    name_blind_index VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE service_accounts (
    id SERIAL PRIMARY KEY, 
    user_id INT NOT NULL, 
    vault_id INT, 
    name VARCHAR NOT NULL,
    encrypted_ecdh_private_key VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE service_account_secrets (
    id SERIAL PRIMARY KEY, 
    service_account_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    name_blind_index VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE users_vaults (
    user_id INT NOT NULL, 
    vault_id INT NOT NULL, 
    ecdh_public_key VARCHAR NOT NULL,
    encrypted_vault_key VARCHAR NOT NULL
);
CREATE TABLE organisations (
    id SERIAL PRIMARY KEY, 
    name VARCHAR,
    created_by_user_id INT NOT NULL
);

CREATE TABLE organisation_users (
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (user_id, organisation_id)
);

CREATE TABLE invitations (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    email VARCHAR NOT NULL,
    invitation_selector VARCHAR NOT NULL,
    invitation_verifier_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   CONSTRAINT fk_organisation
      FOREIGN KEY(organisation_id) 
	  REFERENCES organisations(id)
);

-- Give access to cloak user
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO cloak;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO cloak;

-- Give access to readonly user
GRANT SELECT ON ALL TABLES IN SCHEMA public TO cloak_readonly;
GRANT SELECT ON ALL SEQUENCES IN SCHEMA public TO cloak_readonly;

-- Give access to auth user
GRANT SELECT, INSERT, UPDATE, DELETE ON sessions TO cloak_auth;
GRANT USAGE, SELECT ON sessions_id_seq TO cloak_auth;
GRANT SELECT, INSERT, UPDATE, DELETE ON users TO cloak_auth;
GRANT USAGE, SELECT ON users_id_seq TO cloak_auth;

-- Manage the updated_at column
SELECT updated_at('users');
SELECT updated_at('secrets');
SELECT updated_at('vaults');
SELECT updated_at('service_accounts');
SELECT updated_at('service_account_secrets');
SELECT updated_at('invitations');

-- migrate:down
DROP TABLE users;
DROP TABLE sessions;
DROP TABLE vaults;
DROP TABLE secrets;
DROP TABLE service_accounts;
DROP TABLE service_account_secrets;
DROP TABLE users_vaults;
DROP TABLE organisation_users;
DROP TABLE invitations;
DROP TABLE organisations;

