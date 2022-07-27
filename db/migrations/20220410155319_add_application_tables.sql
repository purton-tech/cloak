-- migrate:up
CREATE TABLE vaults (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_organisation
        FOREIGN KEY(organisation_id) 
        REFERENCES organisations(id)
        ON DELETE CASCADE
);

CREATE TABLE secrets (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    environment_id INT NOT NULL,
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    name_blind_index VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT fk_vault
        FOREIGN KEY(vault_id) 
        REFERENCES vaults(id)
        ON DELETE CASCADE
);

CREATE TABLE service_accounts (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    vault_id INT, 
    environment_id INT,
    name VARCHAR NOT NULL,
    encrypted_ecdh_private_key VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_organisation
        FOREIGN KEY(organisation_id) 
        REFERENCES organisations(id)
        ON DELETE CASCADE
);

CREATE TABLE service_account_secrets (
    id SERIAL PRIMARY KEY, 
    service_account_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    name_blind_index VARCHAR NOT NULL,
    ecdh_public_key VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_service_account
        FOREIGN KEY(service_account_id) 
        REFERENCES service_accounts(id)
        ON DELETE CASCADE
);

CREATE TABLE users_vaults (
    user_id INT NOT NULL, 
    vault_id INT NOT NULL, 
    ecdh_public_key VARCHAR NOT NULL,
    encrypted_vault_key VARCHAR NOT NULL,

    PRIMARY KEY (user_id, vault_id),
    
    CONSTRAINT fk_vault
        FOREIGN KEY(vault_id) 
        REFERENCES vaults(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE TABLE environments (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL,

    CONSTRAINT fk_vault
        FOREIGN KEY (vault_id)
        REFERENCES vaults(id) 
        ON DELETE CASCADE
);

COMMENT ON TABLE environments IS 'Contains the environments of secrets we store in a vault';
COMMENT ON COLUMN environments.vault_id IS 'The vault these environments belong to';
COMMENT ON COLUMN environments.name IS 'A user generated name for the environment';

CREATE TABLE users_environments (
    environment_id INT NOT NULL, 
    user_id INT NOT NULL,
    PRIMARY KEY (environment_id, user_id),

    CONSTRAINT fk_environment
        FOREIGN KEY (environment_id)
        REFERENCES environments(id) 
        ON DELETE CASCADE,

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES users(id) 
        ON DELETE CASCADE
);

-- Give access to application user
GRANT SELECT, INSERT, UPDATE, DELETE ON environments, users_environments, vaults, secrets, service_accounts, service_account_secrets, users_vaults TO application;
GRANT USAGE, SELECT ON environments_id_seq, vaults_id_seq, secrets_id_seq, service_accounts_id_seq, service_account_secrets_id_seq TO application;

-- Give access to readonly user
GRANT SELECT ON environments, users_environments, vaults, secrets, service_accounts, service_account_secrets, users_vaults  TO readonly;
GRANT SELECT ON environments_id_seq, vaults_id_seq, secrets_id_seq, service_accounts_id_seq, service_account_secrets_id_seq TO readonly;

-- Manage the updated_at column
SELECT updated_at('secrets');
SELECT updated_at('vaults');
SELECT updated_at('service_accounts');
SELECT updated_at('service_account_secrets');

-- migrate:down
DROP TABLE secrets;
DROP TABLE service_account_secrets;
DROP TABLE users_vaults;
DROP TABLE service_accounts;
DROP TABLE users_environments;
DROP TABLE environments;
DROP TABLE vaults;

