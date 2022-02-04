CREATE TABLE secrets (
    id SERIAL PRIMARY KEY, 
    vault_id INT NOT NULL, 
    name VARCHAR NOT NULL,
    secret VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

GRANT SELECT, INSERT, UPDATE, DELETE ON secrets TO cloak;
GRANT USAGE, SELECT ON secrets_id_seq TO cloak;

-- Manage the updated_at column
SELECT updated_at('secrets');