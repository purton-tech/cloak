-- migrate:up

-- The very simplest RBAC implementation, the roles get added to the organisation_users table
-- as users are added to an org.

CREATE TYPE role AS ENUM (
    'Administrator', 
    'Collaborator', 
    'SystemAdministrator'
);

/*

-- If required we could extend with permissions

CREATE TYPE permission AS ENUM (
    'InviteUsers'
);

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);

*/

-- migrate:down
DROP TYPE role;