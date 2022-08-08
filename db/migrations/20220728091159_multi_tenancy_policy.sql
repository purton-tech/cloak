-- migrate:up
CREATE FUNCTION current_app_user() RETURNS INTEGER AS 
$$ 
    SELECT
        current_setting(
            'row_level_security.user_id',
            false
        )::integer 
$$ LANGUAGE SQL;
COMMENT ON FUNCTION current_app_user IS 
    'These needs to be set by the application before accessing the database.';

CREATE FUNCTION current_ecdh_public_key() RETURNS TEXT AS 
$$ 
    SELECT
        current_setting(
            'row_level_security.ecdh_public_key',
            true
        )
$$ LANGUAGE SQL;
COMMENT ON FUNCTION current_app_user IS 
    'These needs to be set by the application before accessing the database.';

CREATE FUNCTION get_orgs_for_app_user() RETURNS setof integer AS 
$$ 
DECLARE
    current_key text := current_ecdh_public_key();
BEGIN
    -- raise notice 'Key (%)', current_key;
    -- Is this an API call using the ECDH public key?
    IF current_key IS NOT NULL AND LENGTH(current_key) > 10 THEN
        RETURN QUERY SELECT
            organisation_id
        FROM
            service_accounts
        WHERE
            ecdh_public_key = current_key;
    -- It's a normal call get the current app user
    ELSE
        RETURN QUERY SELECT
            organisation_id
        FROM
            organisation_users
        WHERE
            user_id = current_app_user();
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
COMMENT ON FUNCTION get_orgs_for_app_user IS 
    'All the orgs a user has been invited to.';

CREATE FUNCTION get_orgs_app_user_created() RETURNS setof integer AS 
$$ 
    SELECT
        id
    FROM
        organisations
    WHERE
        created_by_user_id = current_app_user()
$$ LANGUAGE SQL SECURITY DEFINER;
COMMENT ON FUNCTION get_orgs_app_user_created IS 
    'All the orgs a user created.';

CREATE FUNCTION get_users_for_app_user() RETURNS setof integer AS 
$$ 
    SELECT
        user_id
    FROM
        organisation_users
    WHERE
        organisation_id IN (SELECT get_orgs_for_app_user())
$$ LANGUAGE SQL SECURITY DEFINER;
COMMENT ON FUNCTION get_users_for_app_user IS 
    'All the users from all the orgs this user has been invited to.';

-- Only see users we have access to
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON users
    FOR ALL TO application
    USING (
        id IN (SELECT get_users_for_app_user())
    );
COMMENT ON POLICY multi_tenancy_policy ON users IS 
    'A user can see all the users for orgs they have created or been invited to.';

-- We must have been given access to the org or be the orgs creator
ALTER TABLE organisations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON organisations
    FOR ALL TO application
    USING (
        id IN (SELECT get_orgs_for_app_user())
        OR
        created_by_user_id = current_app_user()
    );
COMMENT ON POLICY multi_tenancy_policy ON organisations IS 
    'A user can see all the orgs they have created or been invited to.';

--! We can only attach a user to an org if there is a corresponding invitation
--! or organisations.created_by_user_id matches the user
ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_insert ON organisation_users
    FOR INSERT TO application
    WITH CHECK (
        organisation_id IN (
            SELECT organisation_id FROM invitations 
        )
        OR 
        organisation_id IN (
            SELECT get_orgs_app_user_created()
        )
    );
COMMENT ON POLICY multi_tenancy_policy_insert ON organisation_users IS 
    'A user on connect users with orgs it created or where an invite exists.';

ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_select ON organisation_users
    FOR SELECT TO application
    USING (
        true
        -- Makes a stack depth error
        --organisation_id IN (SELECT get_orgs_for_app_user())
    );
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 
    'Allow the user to see the organisation-users table';

ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_delete ON organisation_users
    FOR DELETE TO application
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 
    'Only disconnect a user from an org if we have access to that org.';

-- Only users who are members of an organsiation can create invites.
ALTER TABLE invitations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON invitations
    FOR ALL TO application
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
COMMENT ON POLICY multi_tenancy_policy ON invitations IS 
    'A users can access inviation from one of the orgs or if it has the same email address';

-- Restrict audit trail access to the organisations a user has access to.
ALTER TABLE audit_trail ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON audit_trail
    FOR ALL TO application
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );

--! Restrict service_accounts access to the organisations a user has access to.
ALTER TABLE service_accounts ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_accounts
    FOR ALL TO application
    USING (
        ecdh_public_key = current_ecdh_public_key()
        OR
        organisation_id IN (SELECT get_orgs_for_app_user())
    );

--! Restrict vaults access to the organisations a user has access to.
ALTER TABLE vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON vaults
    FOR ALL TO application
    USING (
        organisation_id IN (SELECT get_orgs_for_app_user())
    );

-- Tables indirectly connected to the org, i.e. connected to the tables above.

ALTER TABLE users_vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON users_vaults
    FOR ALL TO application
    USING (
        vault_id IN (SELECT vault_id FROM vaults)
        AND
        user_id IN (SELECT get_users_for_app_user())
    );

ALTER TABLE secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON secrets
    FOR ALL TO application
    USING (
        vault_id IN (SELECT vault_id FROM users_vaults)
    );

ALTER TABLE environments ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON environments
    FOR ALL TO application
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
    FOR ALL TO application
    USING (
       service_account_id IN (
            SELECT service_account_id 
            FROM service_accounts
            WHERE organisation_id IN (SELECT get_orgs_for_app_user()))
    );

-- Open up for authentication
CREATE POLICY authentication_policy ON users TO authentication USING (true);
CREATE POLICY authentication_policy ON sessions TO authentication USING (true);

-- Open up for database backup
CREATE POLICY readonly_policy ON audit_trail FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON environments FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON invitations FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON organisation_users FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON organisations FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON roles_permissions FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON schema_migrations FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON secrets FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON service_account_secrets FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON service_accounts FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON sessions FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON users FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON users_environments FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON users_vaults FOR SELECT TO authentication USING (true);
CREATE POLICY readonly_policy ON vaults FOR SELECT TO authentication USING (true);

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

-- Drop auth policies
DROP POLICY authentication_policy ON users;
DROP POLICY authentication_policy ON sessions;

DROP POLICY readonly_policy ON audit_trail;
DROP POLICY readonly_policy ON environments;
DROP POLICY readonly_policy ON invitations;
DROP POLICY readonly_policy ON organisation_users;
DROP POLICY readonly_policy ON organisations;
DROP POLICY readonly_policy ON roles_permissions;
DROP POLICY readonly_policy ON schema_migrations;
DROP POLICY readonly_policy ON secrets;
DROP POLICY readonly_policy ON service_account_secrets;
DROP POLICY readonly_policy ON service_accounts;
DROP POLICY readonly_policy ON sessions;
DROP POLICY readonly_policy ON users;
DROP POLICY readonly_policy ON users_environments;
DROP POLICY readonly_policy ON users_vaults;
DROP POLICY readonly_policy ON vaults;

DROP FUNCTION current_app_user;
DROP FUNCTION current_ecdh_public_key;
DROP FUNCTION get_orgs_for_app_user;
DROP FUNCTION get_users_for_app_user;
DROP FUNCTION get_orgs_app_user_created;