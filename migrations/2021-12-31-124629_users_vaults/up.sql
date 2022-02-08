CREATE TABLE users_vaults (
    user_id INT NOT NULL, 
    vault_id INT NOT NULL, 
    encrypted_vault_key VARCHAR NOT NULL
);

GRANT SELECT, INSERT, UPDATE, DELETE ON users_vaults TO cloak;
GRANT SELECT ON users_vaults TO cloak_readonly;