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
COMMENT ON TABLE vaults IS 'Vaults allow users to divide secrets into logical groupings.';
COMMENT ON COLUMN vaults.organisation_id IS 'Vaults belong to an organisation';
COMMENT ON COLUMN vaults.name IS 'The user supplied name of the vault';

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
COMMENT ON TABLE secrets IS 'Secrets are encrypted name value pairs.';
COMMENT ON COLUMN secrets.vault_id IS 'Secrets belong to vaults';
COMMENT ON COLUMN secrets.environment_id IS 'Secrets a re partioned into environments i.e. Dev, Production, CICD etc.';
COMMENT ON COLUMN secrets.name IS 'The name of the secret encrypted with the vault key';
COMMENT ON COLUMN secrets.secret IS 'The value of the secret encrypted with the vault key';
COMMENT ON COLUMN secrets.name_blind_index IS 'A blind index generated from the secrets name.';

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
COMMENT ON TABLE users_vaults IS 'Connects users to vaults and holds a copy of the vault key encrypted with their AES key.';
COMMENT ON COLUMN users_vaults.ecdh_public_key IS 'An ECDH public key used to encrypt the vaults secrets for this user.';
COMMENT ON COLUMN users_vaults.encrypted_vault_key IS 'A wrapped ECDH private key, used to decrypt the secrets for this user.';

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
COMMENT ON TABLE service_accounts IS 'If a user is a member of a vault they can create a service account which will recieve a copy of the secrets.';
COMMENT ON COLUMN service_accounts.organisation_id IS 'Service accounts belong to organisations.';
COMMENT ON COLUMN service_accounts.vault_id IS 'The vault this service account will recieve secrets from.';
COMMENT ON COLUMN service_accounts.environment_id IS 'The environment in the vault this service account will recieve secrets from.';
COMMENT ON COLUMN service_accounts.name IS 'The name of this service account.';
COMMENT ON COLUMN service_accounts.encrypted_ecdh_private_key IS 'A wrapped ECDH private key, used to decrypt the secrets for this service account.';


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
COMMENT ON TABLE service_account_secrets IS 'When a service account is connected to a vault a copy of the secrets will be stored here.';
COMMENT ON COLUMN service_account_secrets.service_account_id IS 'Service accounts secrets are connect to service accounts.';
COMMENT ON COLUMN service_account_secrets.name IS 'A blind index of the secrets name.';
COMMENT ON COLUMN service_account_secrets.ecdh_public_key IS 'ECDH public key used to encrypt secrets.';

CREATE TABLE environments (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL,

    CONSTRAINT fk_vault
        FOREIGN KEY (vault_id)
        REFERENCES vaults(id) 
        ON DELETE CASCADE
);
COMMENT ON TABLE environments IS 'Vaults are further divided into environments.';
COMMENT ON COLUMN environments.vault_id IS 'Environments are connected to vaults.';
COMMENT ON COLUMN environments.name IS 'A name such as Prod, Dev, CICD etc.';

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
COMMENT ON TABLE users_environments IS 'When we add a user to a vault we can select which environemnts they are allowed to see.';

-- Give access to application user
GRANT SELECT, INSERT, UPDATE, DELETE ON environments, vaults, secrets, service_accounts, service_account_secrets TO application;
GRANT SELECT, INSERT, DELETE ON users_environments, users_vaults TO application;
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

