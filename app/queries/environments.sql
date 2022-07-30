--! get_all(vault_id) { id, name } *
SELECT  
    id, 
    name
FROM 
    environments 
WHERE 
    vault_id = $1
AND 
    id
IN
    (SELECT environment_id 
    FROM
        users_environments
    WHERE
        user_id = current_setting('row_level_security.user_id')::integer)
ORDER BY name

--! connect_environment_to_user(user_id, environment_id)
INSERT INTO users_environments (user_id, environment_id) VALUES($1, $2);

--! setup_environments(vault_id) { id, name } *
INSERT INTO 
    environments (vault_id, name)
VALUES
    ($1, 'Development'),
    ($1, 'Staging'),
    ($1, 'Production')
RETURNING id, name;

--! get_environments_and_vaults() { id, name, vault_name, vault_id } *
SELECT  
    id, 
    name,
    (SELECT name from vaults v WHERE vault_id = v.id) as vault_name,
    vault_id
FROM 
    environments
WHERE
    id
IN
    (SELECT environment_id 
    FROM
        users_environments
    WHERE
        user_id = current_setting('row_level_security.user_id')::integer)
ORDER BY name