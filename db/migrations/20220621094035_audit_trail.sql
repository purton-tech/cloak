-- migrate:up
CREATE TYPE audit_entity AS ENUM ('Vault', 'ServiceAccount', 'UserManagement');
CREATE TYPE audit_action AS ENUM ('AccessSecrets', 'Logon', 'Logout');

CREATE TABLE audit_trail (
    id SERIAL PRIMARY KEY,
    entity audit_entity NOT NULL,
    action audit_action NOT NULL,
    user_id INT NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- IN the unlikely event we delete a user, delete all the audit trail items.
    CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
        REFERENCES users(id)
        ON DELETE CASCADE
);

COMMENT ON TABLE audit_trail IS 'Log all accesses to the system';
COMMENT ON COLUMN audit_trail.entity IS 'The part of the system we are adding an audit entry for';
COMMENT ON COLUMN audit_trail.action IS 'The action committed. i.e. deleting a secret etc.';

-- Grant access
GRANT SELECT, INSERT ON audit_trail TO cloak;
GRANT USAGE, SELECT ON audit_trail_id_seq TO cloak;

GRANT SELECT ON audit_trail TO cloak_readonly;
GRANT SELECT ON audit_trail_id_seq TO cloak_readonly;

-- migrate:down
DROP TABLE audit_trail;
DROP TYPE audit_entity;
DROP TYPE audit_action;
