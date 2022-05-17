-- migrate:up
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

GRANT SELECT, INSERT, UPDATE, DELETE ON users_environments TO cloak;
GRANT SELECT ON users_environments TO cloak_readonly;

COMMENT ON TABLE users_environments IS 'Members of a vault have access to a selection of environments';

-- Cascade deletes, this won't work if there are secrets disconnected from vaults

ALTER TABLE secrets
    ADD CONSTRAINT fk_secret_vault 
    FOREIGN KEY (vault_id) 
    REFERENCES vaults(id) ON DELETE CASCADE;
    
ALTER TABLE service_account_secrets
    ADD CONSTRAINT fk_secrets_service_accounts 
    FOREIGN KEY (service_account_id) 
    REFERENCES service_accounts(id) ON DELETE CASCADE;
    
ALTER TABLE users_vaults
    ADD CONSTRAINT fk_users_vaults_vaults
    FOREIGN KEY (vault_id) 
    REFERENCES vaults(id) ON DELETE CASCADE;

-- migrate:down

DROP TABLE users_environments;

ALTER TABLE secrets DROP CONSTRAINT fk_secret_vault;
ALTER TABLE service_account_secrets DROP CONSTRAINT fk_secrets_service_accounts;
ALTER TABLE users_vaults DROP CONSTRAINT fk_users_vaults_vaults;