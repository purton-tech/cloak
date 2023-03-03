-- migrate:up

-- Open up for authentication
CREATE POLICY authentication_policy ON users TO cloak_authentication USING (true);

-- Open up for database backup
CREATE POLICY readonly_policy ON audit_trail FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON environments FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON invitations FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON organisation_users FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON organisations FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON secrets FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON service_account_secrets FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON service_accounts FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON users FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON users_vaults FOR SELECT TO cloak_readonly USING (true);
CREATE POLICY readonly_policy ON vaults FOR SELECT TO cloak_readonly USING (true);

-- migrate:down


-- Drop auth policies
DROP POLICY authentication_policy ON users;

DROP POLICY readonly_policy ON audit_trail;
DROP POLICY readonly_policy ON environments;
DROP POLICY readonly_policy ON invitations;
DROP POLICY readonly_policy ON organisation_users;
DROP POLICY readonly_policy ON organisations;
DROP POLICY readonly_policy ON secrets;
DROP POLICY readonly_policy ON service_account_secrets;
DROP POLICY readonly_policy ON service_accounts;
DROP POLICY readonly_policy ON users;
DROP POLICY readonly_policy ON users_vaults;
DROP POLICY readonly_policy ON vaults;
