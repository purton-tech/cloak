-- migrate:up

-- The very simplest RBAC implementation, the roles get added to the organisation_users table
-- as users are added to an org.

CREATE TYPE role AS ENUM (
    'Administrator', 
    'Collaborator', 
    'SystemAdministrator'
);

CREATE TYPE permission AS ENUM (
    -- The ManageTeam permission gives the user thee ability to invite team members, 
    -- delete team members and chnage the team name
    'ManageTeam'
);

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);

INSERT INTO roles_permissions VALUES('Administrator', 'ManageTeam');


-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON roles_permissions TO application;

-- Give access to the readonly user
GRANT SELECT ON roles_permissions TO readonly;

-- migrate:down
DROP TABLE roles_permissions;
DROP TYPE role;
DROP TYPE permission;