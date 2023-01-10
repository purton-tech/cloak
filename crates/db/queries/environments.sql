--: Environment()

--! get_all : Environment
SELECT  
    id, 
    name
FROM 
    environments 
WHERE 
    vault_id = :vault_id
AND 
    id
IN
    (SELECT environment_id 
    FROM
        users_environments
    WHERE
        user_id = current_app_user())
ORDER BY name;

--! connect_environment_to_user
INSERT INTO users_environments (user_id, environment_id) VALUES(:user_id, :environment_id);

--! setup_environments
INSERT INTO 
    environments (vault_id, name)
VALUES
    (:vault_id, 'Development'),
    (:vault_id, 'Staging'),
    (:vault_id, 'Production')
RETURNING id, name;

--! get_environments_and_vaults : EnvironmentsAndVault()
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
        user_id = current_app_user())
ORDER BY name;