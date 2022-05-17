-- migrate:up
ALTER TABLE users_vaults ADD PRIMARY KEY (user_id, vault_id);

-- migrate:down
ALTER TABLE users_vaults DROP CONSTRAINT users_vaults_pkey;  