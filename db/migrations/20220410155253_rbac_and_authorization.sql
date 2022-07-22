-- migrate:up

CREATE TYPE role AS ENUM (
    'Administrator', 
    'SystemAdministrator'
);

CREATE TYPE permission AS ENUM (
    'InviteUsers'
);

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);

CREATE TABLE organisation_user_roles (
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL,
    role role NOT NULL,
    PRIMARY KEY (user_id, organisation_id)
);

-- Grant access
-- Give access to cloak user
GRANT SELECT, INSERT, UPDATE, DELETE ON roles_permissions, organisation_user_roles TO application;

-- Give access to readonly user
GRANT SELECT ON roles_permissions, organisation_user_roles TO readonly;

-- migrate:down
DROP TABLE roles_permissions;
DROP TABLE organisation_user_roles;
DROP TYPE permission;
DROP TYPE role;