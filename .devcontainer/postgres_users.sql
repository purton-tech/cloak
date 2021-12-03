-- These users will need to be created in production with the same names
-- All the permissions will need to be setup with the migrations.
CREATE USER keyvault_app LOGIN ENCRYPTED PASSWORD 'testpassword';
CREATE USER keyvault_auth LOGIN ENCRYPTED PASSWORD 'testpassword';
CREATE USER keyvault_readonly LOGIN ENCRYPTED PASSWORD 'testpassword';
-- The migrations users
CREATE USER keyvault_migrations CREATEDB CREATEROLE LOGIN ENCRYPTED PASSWORD 'testpassword';
