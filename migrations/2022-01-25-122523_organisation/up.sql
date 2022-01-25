CREATE TABLE organisations (
    id SERIAL PRIMARY KEY, 
    owning_user_id INT NOT NULL, 
    name VARCHAR
);

CREATE TABLE organisation_users (
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL
);

INSERT INTO organisations (owning_user_id)
    SELECT id AS owning_user_id FROM users;