-- migrate:up

CREATE POLICY multi_tenancy_policy ON audit_trail FOR ALL TO application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);

CREATE POLICY multi_tenancy_policy ON environments FOR ALL TO application
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

CREATE POLICY multi_tenancy_policy ON invitations FOR ALL TO application
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
);

-- organisation_users
CREATE POLICY multi_tenancy_policy_insert ON organisation_users FOR INSERT TO application
WITH CHECK (
    organisation_id IN (
        SELECT organisation_id FROM invitations 
    )
    OR 
    organisation_id IN (
        SELECT get_orgs_app_user_created()
    )
);

CREATE POLICY multi_tenancy_policy_select ON organisation_users FOR SELECT TO application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);

CREATE POLICY multi_tenancy_policy_delete ON organisation_users FOR DELETE TO application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);

CREATE POLICY multi_tenancy_policy ON organisations FOR ALL TO application
USING (
    id IN (SELECT get_orgs_for_app_user())
    OR
    created_by_user_id = current_app_user()
);

CREATE POLICY multi_tenancy_policy ON service_accounts FOR ALL TO application
USING (
    ecdh_public_key = current_ecdh_public_key()
    OR
    organisation_id IN (SELECT get_orgs_for_app_user())
);

CREATE POLICY multi_tenancy_policy ON service_account_secrets FOR ALL TO application
USING (
    service_account_id IN (
        SELECT service_account_id 
        FROM service_accounts
        WHERE organisation_id IN (SELECT get_orgs_for_app_user()))
);

CREATE POLICY multi_tenancy_policy ON secrets FOR ALL TO application
USING (
    vault_id IN (SELECT vault_id FROM users_vaults)
);

CREATE POLICY multi_tenancy_policy ON users_vaults FOR ALL TO application
USING (
    vault_id IN (SELECT vault_id FROM vaults)
    AND
    user_id IN (SELECT get_users_for_app_user())
);

CREATE POLICY multi_tenancy_policy ON users FOR ALL TO application
USING (id IN (SELECT get_users_for_app_user()));

CREATE POLICY multi_tenancy_policy ON vaults FOR ALL TO application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);

COMMENT ON POLICY multi_tenancy_policy ON invitations IS 
    'A users can access inviation from one of the orgs or if it has the same email address';
COMMENT ON POLICY multi_tenancy_policy_insert ON organisation_users IS 
    'A user on connect users with orgs it created or where an invite exists.';
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 
    'Only disconnect a user from an org if we have access to that org.';
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 
    'Allow the user to see the organisation-users table';
COMMENT ON POLICY multi_tenancy_policy ON organisations IS 
    'A user can see all the orgs they have created or been invited to.';
COMMENT ON POLICY multi_tenancy_policy ON users IS 
    'A user can see all the users for orgs they have created or been invited to.';

-- migrate:down
DROP POLICY multi_tenancy_policy ON audit_trail;
DROP POLICY multi_tenancy_policy ON environments;
DROP POLICY multi_tenancy_policy ON invitations;
DROP POLICY multi_tenancy_policy ON organisations;
DROP POLICY multi_tenancy_policy_insert ON organisation_users;
DROP POLICY multi_tenancy_policy_select ON organisation_users;
DROP POLICY multi_tenancy_policy_delete ON organisation_users;
DROP POLICY multi_tenancy_policy ON service_accounts;
DROP POLICY multi_tenancy_policy ON service_account_secrets;
DROP POLICY multi_tenancy_policy ON secrets;
DROP POLICY multi_tenancy_policy ON users_vaults;
DROP POLICY multi_tenancy_policy ON users;
DROP POLICY multi_tenancy_policy ON vaults;
