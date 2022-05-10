-- migrate:up
ALTER TABLE secrets ADD COLUMN environment_id INT NOT NULL DEFAULT 0;

CREATE TABLE environments (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL
);

COMMENT ON TABLE environments IS 'Contains the environments of secrets we store in a vault';
COMMENT ON COLUMN environments.vault_id IS 'The vault these environments belong to';
COMMENT ON COLUMN environments.name IS 'A user generated name for the environment';

GRANT SELECT, INSERT, UPDATE, DELETE ON environments TO cloak;
GRANT USAGE, SELECT ON environments_id_seq TO cloak;

-- migrate:down
ALTER TABLE secrets DROP COLUMN environment_id;
DROP TABLE environments;