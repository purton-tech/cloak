-- migrate:up

--! We only want the application role to be restricted.
ALTER ROLE authentication BYPASSRLS; 
ALTER ROLE readonly BYPASSRLS; 

CREATE FUNCTION current_app_user() RETURNS INTEGER AS 
$$ 
    SELECT
        NULLIF(
        current_setting(
            'row_level_security.user_id',
            TRUE
        ),
        ''
        )::INTEGER 
$$ LANGUAGE SQL;

CREATE FUNCTION rls_bypass_check_if_we_are_creator(_organisation_id INTEGER) RETURNS bool AS 
$$ 
    SELECT
        EXISTS(
            SELECT id 
            FROM 
                organisations 
            WHERE 
                created_by_user_id =  current_app_user()
        ) 
$$ LANGUAGE SQL SECURITY INVOKER;

CREATE FUNCTION rls_bypass_org_check(_organisation_id INTEGER) RETURNS bool AS 
$$ 
    SELECT
        EXISTS(
            SELECT
                1
            FROM
                organisation_users
            WHERE
                user_id = current_app_user()
                AND
                organisation_id = _organisation_id
        ) 
$$ LANGUAGE SQL SECURITY INVOKER;

CREATE FUNCTION org_check(_organisation_id INTEGER) RETURNS bool AS 
$$ 
    SELECT
        EXISTS(
            SELECT
                1
            FROM
                organisations
            WHERE
                id = _organisation_id
        ) 
$$ LANGUAGE SQL;

-- We must have been given access to the org or be the orgs creator
ALTER TABLE organisations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON organisations
    FOR ALL
    USING (
        -- Either we have been given access to this org
        rls_bypass_org_check(id)
        OR
        -- Or we are the creator and we haven't connected it yet
        created_by_user_id = current_app_user()
    );

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
        rls_bypass_check_if_we_are_creator(organisation_id)
    );

ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_select ON organisation_users
    FOR SELECT
    USING (
        true
        -- Makes a stack depth error
        --rls_bypass_org_check(organisation_id)
    );

ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_delete ON organisation_users
    FOR DELETE
    USING (
        org_check(organisation_id)
    );

-- Only users who are members of an organsiation can create invites.
ALTER TABLE invitations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_insert ON invitations
    FOR INSERT
    WITH CHECK (
        -- Is this invitation from an org we have access to?
        org_check(organisation_id)
        -- Implement TeamManager permission somehow.
    );

CREATE POLICY multi_tenancy_policy_select ON invitations
    FOR SELECT
    USING (
        -- Is this invitation from an org we have access to?
        org_check(organisation_id)
        -- If the invite is not accepted yet, then we check against the users email address.
        OR (
            email IN (
                SELECT email FROM users WHERE id = current_app_user()
            )
        )
    );

CREATE POLICY multi_tenancy_policy_delete ON invitations
    FOR DELETE
    USING (
        -- Is this invitation from an org we have access to?
        org_check(organisation_id)
    );

-- Restrict audit trail access to the organisations a user has access to.
ALTER TABLE audit_trail ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON audit_trail
    FOR ALL
    USING (
        org_check(organisation_id)
    );

--! Restrict service_accounts access to the organisations a user has access to.
ALTER TABLE service_accounts ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_accounts
    FOR ALL
    USING (
        org_check(organisation_id)
    );

--! Restrict vaults access to the organisations a user has access to.
ALTER TABLE vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON vaults
    FOR ALL
    USING (
        org_check(organisation_id)
    );

-- Tables indirectly connected to the org, i.e. connected to the tables above.

ALTER TABLE users_vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON users_vaults
    FOR ALL
    USING (
        vault_id IN (SELECT vault_id FROM vaults)
    );

ALTER TABLE secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON secrets
    FOR ALL
    USING (
        vault_id IN (SELECT vault_id FROM users_vaults)
    );

ALTER TABLE environments ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_select ON environments
    FOR SELECT
    USING (
       vault_id IN (SELECT vault_id FROM users_vaults 
       WHERE user_id = current_app_user())
       --AND
       --id IN (SELECT environment_id FROM users_environments
       --WHERE user_id = current_setting('row_level_security.user_id')::integer)
    );

ALTER TABLE environments ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_insert ON environments
    FOR INSERT
    WITH CHECK (
       vault_id IN (SELECT vault_id FROM users_vaults 
       WHERE user_id = current_app_user())
    );

ALTER TABLE service_account_secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_account_secrets
    FOR ALL
    USING (
       service_account_id IN (SELECT service_account_id FROM service_accounts)
    );

-- migrate:down
ALTER TABLE organisations DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON organisations;
ALTER TABLE organisation_users DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy_insert ON organisation_users;
DROP POLICY multi_tenancy_policy_select ON organisation_users;
DROP POLICY multi_tenancy_policy_delete ON organisation_users;
ALTER TABLE invitations DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy_insert ON invitations;
DROP POLICY multi_tenancy_policy_delete ON invitations;
DROP POLICY multi_tenancy_policy_select ON invitations;
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
DROP POLICY multi_tenancy_policy_insert ON environments;
DROP POLICY multi_tenancy_policy_select ON environments;
ALTER TABLE service_account_secrets DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON service_account_secrets;

DROP FUNCTION current_app_user;
DROP FUNCTION rls_bypass_org_check;
DROP FUNCTION rls_bypass_check_if_we_are_creator;
DROP FUNCTION org_check;

ALTER ROLE authentication NOBYPASSRLS; 
ALTER ROLE readonly NOBYPASSRLS; 