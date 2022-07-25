-- migrate:up

-- The very simplest RBAC implementation, the roles get added to the organisation_users table
-- as users are added to an org.

CREATE TYPE role AS ENUM (
    'Administrator', 
    'Collaborator', 
    'SystemAdministrator'
);

CREATE TYPE permission AS ENUM (
    'CanInviteUsers'
);

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);

INSERT INTO roles_permissions VALUES('Administrator', 'CanInviteUsers');

-- migrate:down
DROP TABLE roles_permissions;
DROP TYPE role;
DROP TYPE permission;