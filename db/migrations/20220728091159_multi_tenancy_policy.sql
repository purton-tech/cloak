-- migrate:up

--! We only want the application role to be restricted.
ALTER ROLE authentication BYPASSRLS; 
ALTER ROLE readonly BYPASSRLS; 

-- Tables connected directly to the organisation

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
            SELECT id 
            FROM 
                organisations 
            WHERE 
                created_by_user_id =  current_setting('row_level_security.user_id')::integer
        )
    );

-- Ideally we don't want the user to select organisation_users for anyone, but how do we do that 
-- without recursion
ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_select ON organisation_users
    FOR SELECT
    USING (
        true
    );

-- Ideally we don't want the user to delete organisation_users for anyone, but how do we do that 
-- without recursion
ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_delete ON organisation_users
    FOR DELETE
    USING (
        true
    );

-- Only users who are members of an organsiation can create invites.
ALTER TABLE invitations ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_insert ON invitations
    FOR INSERT
    WITH CHECK (
        -- Is this invitation from an org we have access to?
        organisation_id IN (
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_setting('row_level_security.user_id')::integer
        )
        -- Implement TeamManager permission somehow.
    );

CREATE POLICY multi_tenancy_policy_select ON invitations
    FOR SELECT
    USING (
        -- Is this invitation from an org we have access to?
        organisation_id IN (
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_setting('row_level_security.user_id')::integer
        )
        -- If the invite is not accepted yet, then we check against the users email address.
        OR (
            email IN (
                SELECT email FROM users WHERE id = current_setting('row_level_security.user_id')::integer
            )
        )
    );

CREATE POLICY multi_tenancy_policy_delete ON invitations
    FOR DELETE
    USING (
        -- Is this invitation from an org we have access to?
        organisation_id IN (
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_setting('row_level_security.user_id')::integer
        )
    );

-- Restrict audit trail access to the organisations a user has access to.
ALTER TABLE audit_trail ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON audit_trail
    FOR ALL
    USING (
        organisation_id IN (
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_setting('row_level_security.user_id')::integer
        )
    );

--! Restrict service_accounts access to the organisations a user has access to.
ALTER TABLE service_accounts ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_accounts
    FOR ALL
    USING (
        organisation_id IN (
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_setting('row_level_security.user_id')::integer
        )
    );

--! Restrict vaults access to the organisations a user has access to.
ALTER TABLE vaults ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON vaults
    FOR ALL
    USING (
        organisation_id IN (
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_setting('row_level_security.user_id')::integer
        )
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
       WHERE user_id = current_setting('row_level_security.user_id')::integer)
       --AND
       --id IN (SELECT environment_id FROM users_environments
       --WHERE user_id = current_setting('row_level_security.user_id')::integer)
    );

ALTER TABLE environments ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy_insert ON environments
    FOR INSERT
    WITH CHECK (
       vault_id IN (SELECT vault_id FROM users_vaults 
       WHERE user_id = current_setting('row_level_security.user_id')::integer)
    );

ALTER TABLE service_account_secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_account_secrets
    FOR ALL
    USING (
       service_account_id IN (SELECT service_account_id FROM service_accounts)
    );

-- migrate:down
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

ALTER ROLE authentication NOBYPASSRLS; 
ALTER ROLE readonly NOBYPASSRLS; 