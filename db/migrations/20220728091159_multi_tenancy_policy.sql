-- migrate:up

-- Tables connected directly to the organisation

--ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
--CREATE POLICY multi_tenancy_policy ON organisation_users
--    FOR ALL
--    USING (
--        user_id = current_setting('row_level_security.user_id')::integer
--    );

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
CREATE POLICY multi_tenancy_policy ON environments
    FOR ALL
    USING (
       vault_id IN (SELECT vault_id FROM users_vaults)
    );

ALTER TABLE service_account_secrets ENABLE ROW LEVEL SECURITY;
CREATE POLICY multi_tenancy_policy ON service_account_secrets
    FOR ALL
    USING (
       service_account_id IN (SELECT service_account_id FROM service_accounts)
    );

-- migrate:down

--ALTER TABLE organisation_users DISABLE ROW LEVEL SECURITY;
--DROP POLICY multi_tenancy_policy ON organisation_users;
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

