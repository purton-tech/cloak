CREATE TABLE organisations (
    id SERIAL PRIMARY KEY, 
    name VARCHAR,
    created_by_user_id INT NOT NULL
);

CREATE TABLE organisation_users (
    user_id INT NOT NULL, 
    is_admin BOOLEAN NOT NULL DEFAULT false,
    organisation_id INT NOT NULL
);

-- Reset the database, we changed the key protocol.
DELETE FROM organisation_users;
DELETE FROM organisations;
DELETE FROM secrets;
DELETE FROM service_account_secrets;
DELETE FROM service_accounts;
DELETE FROM sessions;
DELETE FROM users;
DELETE FROM users_vaults;
DELETE FROM vaults;