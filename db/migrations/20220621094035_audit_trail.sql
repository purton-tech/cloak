-- migrate:up
CREATE TYPE audit_access_type AS ENUM (
    'CLI', 
    'ServiceAccount', 
    'Web'
);

CREATE TYPE audit_action AS ENUM (
    'AddMember', 
    'DeleteMember', 
    'AddSecret',
    'DeleteSecret',
    'AccessSecrets',
    'NewServiceAccount',
    'DeleteServiceAccount',
    'ConnectServiceAccount',
    'CreateInvite',
    'RemoveTeamMember',
    'CreateVault',
    'DeleteVault'
);

CREATE TABLE audit_trail (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL, 
    access_type audit_access_type NOT NULL,
    action audit_action NOT NULL,
    description VARCHAR NOT NULL,
    organisation_id INT NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_organisation
        FOREIGN KEY(organisation_id) 
        REFERENCES organisations(id)
        ON DELETE CASCADE
);

COMMENT ON TABLE audit_trail IS 'Log all accesses to the system';
COMMENT ON COLUMN audit_trail.user_id IS 'The user that accessed the system';
COMMENT ON COLUMN audit_trail.access_type IS 'How was the system accessed i.e. by the CLI or web interface etc.';
COMMENT ON COLUMN audit_trail.action IS 'The action committed. i.e. deleting a secret etc.';
COMMENT ON COLUMN audit_trail.description IS 'A text description of what happened';

-- Grant access
GRANT SELECT, INSERT ON audit_trail TO application;
GRANT USAGE, SELECT ON audit_trail_id_seq TO application;

GRANT SELECT ON audit_trail TO readonly;
GRANT SELECT ON audit_trail_id_seq TO readonly;

-- migrate:down
DROP TABLE audit_trail;
DROP TYPE audit_access_type;
DROP TYPE audit_action;
