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

INSERT INTO organisations (created_by_user_id)
    SELECT id AS created_by_user_id FROM users;

INSERT INTO organisation_users (user_id, organisation_id, is_admin)
    SELECT created_by_user_id AS user_id, id AS organisation_id, true FROM organisations;