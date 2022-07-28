-- migrate:up
ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit_trail ENABLE ROW LEVEL SECURITY;
ALTER TABLE vaults ENABLE ROW LEVEL SECURITY;
ALTER TABLE service_accounts ENABLE ROW LEVEL SECURITY;

-- Tables connected directly to the organisation
CREATE POLICY multi_tenancy_policy ON organisation_users
    FOR ALL
    USING (
        user_id = current_setting('row_level_security.user_id')::integer
    );

CREATE POLICY multi_tenancy_policy ON audit_trail
    FOR ALL
    USING (
        organisation_id IN (SELECT organisation_id FROM organisation_users)
    );

CREATE POLICY multi_tenancy_policy ON service_accounts
    FOR ALL
    USING (
        organisation_id IN (SELECT organisation_id FROM organisation_users)
    );

CREATE POLICY multi_tenancy_policy ON vaults
    FOR ALL
    USING (
        organisation_id IN (SELECT organisation_id FROM organisation_users)
    );

-- Tables indirectly connected to the org, i.e. connected to the tables above.

-- ALTER TABLE users_vaults ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE secrets ENABLE ROW LEVEL SECURITY;
-- CREATE POLICY multi_tenancy_policy ON users_vaults
--    FOR ALL
--    USING (
--        user_id = current_setting('row_level_security.user_id')::integer
--    );

-- CREATE POLICY multi_tenancy_policy ON secrets
--    FOR ALL
--    USING (
--        vault_id IN (SELECT vault_id FROM users_vaults)
--    );

-- CREATE POLICY multi_tenancy_policy ON environments
--    FOR ALL
--    USING (
--        vault_id IN (SELECT vault_id FROM users_vaults)
--    );

-- secrets via vault_id
-- environments via vault_id
-- service_account_secrets via service_account_id

-- migrate:down
ALTER TABLE organisation_users DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON organisation_users;
ALTER TABLE audit_trail DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON audit_trail;
ALTER TABLE service_accounts DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON service_accounts;
ALTER TABLE vaults DISABLE ROW LEVEL SECURITY;
DROP POLICY multi_tenancy_policy ON vaults;

