-- migrate:up

--! We only want the application role to be restricted.
ALTER ROLE authentication BYPASSRLS; 
ALTER ROLE readonly BYPASSRLS; 

CREATE FUNCTION current_app_user() RETURNS INTEGER AS 
$$ 
    SELECT
        current_setting(
            'row_level_security.user_id',
            false
        )::integer 
$$ LANGUAGE SQL;
COMMENT ON FUNCTION current_app_user IS 'These needs to be set by the application before accessing the database.';

CREATE FUNCTION get_orgs_for_app_user() RETURNS setof integer AS 
$$ 
    SELECT
        organisation_id
    FROM
        organisation_users
    WHERE
        user_id = current_app_user()
$$ LANGUAGE SQL SECURITY INVOKER;
COMMENT ON FUNCTION get_orgs_for_app_user IS 'All the orgs a user has been invited to.';

CREATE FUNCTION get_orgs_app_user_created() RETURNS setof integer AS 
$$ 
    SELECT
        id
    FROM
        organisations
    WHERE
        created_by_user_id = current_app_user()
$$ LANGUAGE SQL SECURITY INVOKER;
COMMENT ON FUNCTION get_orgs_app_user_created IS 'All the orgs a user created.';

CREATE FUNCTION get_users_for_app_user() RETURNS setof integer AS 
$$ 
    SELECT
        user_id
    FROM
        organisation_users
    WHERE
        organisation_id IN (SELECT get_orgs_for_app_user())
$$ LANGUAGE SQL SECURITY INVOKER;
COMMENT ON FUNCTION get_users_for_app_user IS 'All the users from all the orgs this user has been invited to.';

-- Only see users we have access to
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON users
    FOR ALL
    USING (
        id IN (SELECT get_users_for_app_user())
    );
COMMENT ON POLICY multi_tenancy_policy ON users IS 'A user can see all the users for orgs they have created or been invited to.';

-- We must have been given access to the org or be the orgs creator
ALTER TABLE organisations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON organisations
    FOR ALL
    USING (
        id IN (SELECT get_orgs_for_app_user())
        OR
        created_by_user_id = current_app_user()
    );
COMMENT ON POLICY multi_tenancy_policy ON organisations IS 'A user can see all the orgs they have created or been invited to.';

--! We can only attach a user to an org if there is a corresponding invitation
--! or organisations.created_by_user_id matches the user
ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_insert ON organisation_users
    FOR INSERT
    WITH CHECK (
        organisation_id IN (
            SELECT organisation_id FROM invitations 
        )
        OR 
        organisation_id IN (
            SELECT get_orgs_app_user_created()
        )
    );
COMMENT ON POLICY multi_tenancy_policy_insert ON organisation_users IS 'A user on connect users with orgs it created or where an invite exists.';

ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_select ON organisation_users
    FOR SELECT
    USING (
        true
        -- Makes a stack depth error
        --organisation_id IN (SELECT get_orgs_for_app_user())
    );
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 'Allow the user to see the organisation-users table';

ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_delete ON organisation_users
    FOR DELETE
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 'Only disconnect a user from an org if we have access to that org.';

-- Only users who are members of an organsiation can create invites.
ALTER TABLE invitations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON invitations
    FOR ALL
    USING (
        -- Is this invitation from an org we have access to?
        organisation_id IN (SELECT get_orgs_for_app_user())
        -- If the invite is not accepted yet, then we check against the users email address.
        OR (
            email IN (
                SELECT email FROM users WHERE id = current_app_user()
            )
        )
    )
    WITH CHECK (
        -- Is this invitation from an org we have access to?
        organisation_id IN (SELECT get_orgs_for_app_user())
        -- Implement TeamManager permission somehow.
    );
COMMENT ON POLICY multi_tenancy_policy ON invitations IS 'A users can access inviation from one of the orgs or if it has the same email address';

-- Restrict audit trail access to the organisations a user has access to.
ALTER TABLE audit_trail ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON audit_trail
    FOR ALL
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );

--! Restrict service_accounts access to the organisations a user has access to.
ALTER TABLE service_accounts ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_accounts
    FOR ALL
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );

--! Restrict vaults access to the organisations a user has access to.
ALTER TABLE vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON vaults
    FOR ALL
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );

-- Tables indirectly connected to the org, i.e. connected to the tables above.

ALTER TABLE users_vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON users_vaults
    FOR ALL
    USING (
        vault_id IN (SELECT vault_id FROM vaults)
        AND
        user_id IN (SELECT get_users_for_app_user())
    );

ALTER TABLE secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON secrets
    FOR ALL
    USING (
        vault_id IN (SELECT vault_id FROM users_vaults)
    );

ALTER TABLE environments ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON environments
    FOR ALL
    USING (
       vault_id IN (SELECT vault_id FROM users_vaults 
       WHERE user_id = current_app_user())
       --AND
       --id IN (SELECT environment_id FROM users_environments
       --WHERE user_id = current_setting('row_level_security.user_id')::integer)
    )
    WITH CHECK (
       vault_id IN (SELECT vault_id FROM users_vaults 
       WHERE user_id = current_app_user())
    );

ALTER TABLE service_account_secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_account_secrets
    FOR ALL
    USING (
       service_account_id IN (
            SELECT service_account_id 
            FROM service_accounts
            WHERE organisation_id IN (SELECT get_orgs_for_app_user()))
    );

-- migrate:down
ALTER TABLE users DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON users;
ALTER TABLE organisations DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON organisations;
ALTER TABLE organisation_users DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy_insert ON organisation_users;
DROP POLICY multi_tenancy_policy_select ON organisation_users;
DROP POLICY multi_tenancy_policy_delete ON organisation_users;
ALTER TABLE invitations DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON invitations;
ALTER TABLE audit_trail DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON audit_trail;
ALTER TABLE service_accounts DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON service_accounts;
ALTER TABLE vaults DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON vaults;
ALTER TABLE users_vaults DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON users_vaults;
ALTER TABLE secrets DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON secrets;
ALTER TABLE environments DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON environments;
ALTER TABLE service_account_secrets DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON service_account_secrets;

DROP FUNCTION current_app_user;
DROP FUNCTION get_orgs_for_app_user;
DROP FUNCTION get_users_for_app_user;
DROP FUNCTION get_orgs_app_user_created;

ALTER ROLE authentication NOBYPASSRLS; 
ALTER ROLE readonly NOBYPASSRLS; 