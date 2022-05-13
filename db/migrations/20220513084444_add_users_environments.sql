-- migrate:up
CREATE TABLE users_environments (
    environment_id INT NOT NULL, 
    user_id INT NOT NULL,
    PRIMARY KEY (environment_id, user_id)
);

GRANT SELECT, INSERT, UPDATE, DELETE ON users_environments TO cloak;
GRANT SELECT ON users_environments TO cloak_readonly;

COMMENT ON TABLE users_environments IS 'Members of a vault have access to a selection of environments';

-- migrate:down

DROP TABLE users_environments;

