SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: audit_access_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.audit_access_type AS ENUM (
    'CLI',
    'ServiceAccount',
    'Web'
);


--
-- Name: audit_action; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.audit_action AS ENUM (
    'AddMember',
    'DeleteMember',
    'AddSecret',
    'DeleteSecret',
    'AccessSecrets',
    'NewServiceAccount',
    'DeleteServiceAccount',
    'ConnectServiceAccount',
    'CreateInvite',
    'RemoveTeamMember',
    'CreateVault',
    'DeleteVault'
);


--
-- Name: permission; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.permission AS ENUM (
    'ManageTeam'
);


--
-- Name: TYPE permission; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.permission IS 'A permission gives the user the ability to do something. i.e. Manage users.';


--
-- Name: role; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.role AS ENUM (
    'Administrator',
    'Collaborator',
    'SystemAdministrator'
);


--
-- Name: TYPE role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TYPE public.role IS 'Users have roles, they can be managers or administrators etc.';


--
-- Name: create_org(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.create_org() RETURNS integer
    LANGUAGE sql
    AS $$
    INSERT INTO users (
        email,
        master_password_hash,
        protected_symmetric_key,
        protected_ecdsa_private_key,
        ecdsa_public_key,
        protected_ecdh_private_key,
        ecdh_public_key
    ) VALUES (
        random()::text,
        'NOT SET',
        'NOT SET',
        'NOT SET',
        'NOT SET',
        'NOT SET',
        'NOT SET'
    ) RETURNING id;
$$;


--
-- Name: current_app_user(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.current_app_user() RETURNS integer
    LANGUAGE sql
    AS $$
    SELECT
        current_setting(
            'row_level_security.user_id',
            false
        )::integer
$$;


--
-- Name: FUNCTION current_app_user(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.current_app_user() IS 'These needs to be set by the application before accessing the database.';


--
-- Name: get_orgs_app_user_created(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_orgs_app_user_created() RETURNS SETOF integer
    LANGUAGE sql
    AS $$
    SELECT
        id
    FROM
        organisations
    WHERE
        created_by_user_id = current_app_user()
$$;


--
-- Name: FUNCTION get_orgs_app_user_created(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_orgs_app_user_created() IS 'All the orgs a user created.';


--
-- Name: get_orgs_for_app_user(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_orgs_for_app_user() RETURNS SETOF integer
    LANGUAGE sql
    AS $$
    SELECT
        organisation_id
    FROM
        organisation_users
    WHERE
        user_id = current_app_user()
$$;


--
-- Name: FUNCTION get_orgs_for_app_user(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_orgs_for_app_user() IS 'All the orgs a user has been invited to.';


--
-- Name: get_users_for_app_user(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.get_users_for_app_user() RETURNS SETOF integer
    LANGUAGE sql
    AS $$
    SELECT
        user_id
    FROM
        organisation_users
    WHERE
        organisation_id IN (SELECT get_orgs_for_app_user())
$$;


--
-- Name: FUNCTION get_users_for_app_user(); Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON FUNCTION public.get_users_for_app_user() IS 'All the users from all the orgs this user has been invited to.';


--
-- Name: set_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


--
-- Name: updated_at(regclass); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl);
END;
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: audit_trail; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.audit_trail (
    id integer NOT NULL,
    user_id integer NOT NULL,
    access_type public.audit_access_type NOT NULL,
    action public.audit_action NOT NULL,
    description character varying NOT NULL,
    organisation_id integer NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE audit_trail; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.audit_trail IS 'Log all accesses to the system';


--
-- Name: COLUMN audit_trail.user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.audit_trail.user_id IS 'The user that accessed the system';


--
-- Name: COLUMN audit_trail.access_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.audit_trail.access_type IS 'How was the system accessed i.e. by the CLI or web interface etc.';


--
-- Name: COLUMN audit_trail.action; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.audit_trail.action IS 'The action committed. i.e. deleting a secret etc.';


--
-- Name: COLUMN audit_trail.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.audit_trail.description IS 'A text description of what happened';


--
-- Name: audit_trail_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.audit_trail_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: audit_trail_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.audit_trail_id_seq OWNED BY public.audit_trail.id;


--
-- Name: environments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.environments (
    id integer NOT NULL,
    vault_id integer NOT NULL,
    name character varying NOT NULL
);


--
-- Name: TABLE environments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.environments IS 'Vaults are further divided into environments.';


--
-- Name: COLUMN environments.vault_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environments.vault_id IS 'Environments are connected to vaults.';


--
-- Name: COLUMN environments.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environments.name IS 'A name such as Prod, Dev, CICD etc.';


--
-- Name: environments_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.environments_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: environments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.environments_id_seq OWNED BY public.environments.id;


--
-- Name: invitations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.invitations (
    id integer NOT NULL,
    organisation_id integer NOT NULL,
    email character varying NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    roles public.role[] NOT NULL,
    invitation_selector character varying NOT NULL,
    invitation_verifier_hash character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE invitations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.invitations IS 'Invitations are generated so users can join teams (organisations)';


--
-- Name: COLUMN invitations.organisation_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.invitations.organisation_id IS 'The organisation that the user will join if they acccept the invite';


--
-- Name: COLUMN invitations.email; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.invitations.email IS 'After we lookup the invite we check that the hash is correct';


--
-- Name: COLUMN invitations.roles; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.invitations.roles IS 'The RBAC privelages the user will receive on joining the team (organisation).';


--
-- Name: COLUMN invitations.invitation_selector; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.invitations.invitation_selector IS 'To avoid timing attacks the inviation secret is split into a lookup then a verfication.';


--
-- Name: invitations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.invitations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: invitations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.invitations_id_seq OWNED BY public.invitations.id;


--
-- Name: organisation_users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.organisation_users (
    user_id integer NOT NULL,
    organisation_id integer NOT NULL,
    roles public.role[] NOT NULL
);


--
-- Name: TABLE organisation_users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.organisation_users IS 'A User can belong to multiple organisations (teams).';


--
-- Name: COLUMN organisation_users.roles; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organisation_users.roles IS 'The RBAC privelages the user has for this team.';


--
-- Name: organisations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.organisations (
    id integer NOT NULL,
    name character varying,
    created_by_user_id integer NOT NULL
);


--
-- Name: TABLE organisations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.organisations IS 'An organisation is created for everyone that signs up. It could also have been called teams.';


--
-- Name: COLUMN organisations.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organisations.name IS 'The name of the organisation i.e. Microsoft or perhaps a persons name';


--
-- Name: COLUMN organisations.created_by_user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organisations.created_by_user_id IS 'The action committed. i.e. deleting a secret etc.';


--
-- Name: organisations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.organisations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: organisations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.organisations_id_seq OWNED BY public.organisations.id;


--
-- Name: roles_permissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.roles_permissions (
    role public.role NOT NULL,
    permission public.permission NOT NULL
);


--
-- Name: TABLE roles_permissions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.roles_permissions IS 'Maps roles to permissions. i.e. a role can have multiple permissions.';


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(255) NOT NULL
);


--
-- Name: secrets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.secrets (
    id integer NOT NULL,
    vault_id integer NOT NULL,
    environment_id integer NOT NULL,
    name character varying NOT NULL,
    secret character varying NOT NULL,
    name_blind_index character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE secrets; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.secrets IS 'Secrets are encrypted name value pairs.';


--
-- Name: COLUMN secrets.vault_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.secrets.vault_id IS 'Secrets belong to vaults';


--
-- Name: COLUMN secrets.environment_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.secrets.environment_id IS 'Secrets a re partioned into environments i.e. Dev, Production, CICD etc.';


--
-- Name: COLUMN secrets.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.secrets.name IS 'The name of the secret encrypted with the vault key';


--
-- Name: COLUMN secrets.secret; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.secrets.secret IS 'The value of the secret encrypted with the vault key';


--
-- Name: COLUMN secrets.name_blind_index; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.secrets.name_blind_index IS 'A blind index generated from the secrets name.';


--
-- Name: secrets_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.secrets_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: secrets_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.secrets_id_seq OWNED BY public.secrets.id;


--
-- Name: service_account_secrets; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.service_account_secrets (
    id integer NOT NULL,
    service_account_id integer NOT NULL,
    name character varying NOT NULL,
    secret character varying NOT NULL,
    name_blind_index character varying NOT NULL,
    ecdh_public_key character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE service_account_secrets; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.service_account_secrets IS 'When a service account is connected to a vault a copy of the secrets will be stored here.';


--
-- Name: COLUMN service_account_secrets.service_account_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_account_secrets.service_account_id IS 'Service accounts secrets are connect to service accounts.';


--
-- Name: COLUMN service_account_secrets.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_account_secrets.name IS 'A blind index of the secrets name.';


--
-- Name: COLUMN service_account_secrets.ecdh_public_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_account_secrets.ecdh_public_key IS 'ECDH public key used to encrypt secrets.';


--
-- Name: service_account_secrets_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.service_account_secrets_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: service_account_secrets_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.service_account_secrets_id_seq OWNED BY public.service_account_secrets.id;


--
-- Name: service_accounts; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.service_accounts (
    id integer NOT NULL,
    organisation_id integer NOT NULL,
    vault_id integer,
    environment_id integer,
    name character varying NOT NULL,
    encrypted_ecdh_private_key character varying NOT NULL,
    ecdh_public_key character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE service_accounts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.service_accounts IS 'If a user is a member of a vault they can create a service account which will recieve a copy of the secrets.';


--
-- Name: COLUMN service_accounts.organisation_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_accounts.organisation_id IS 'Service accounts belong to organisations.';


--
-- Name: COLUMN service_accounts.vault_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_accounts.vault_id IS 'The vault this service account will recieve secrets from.';


--
-- Name: COLUMN service_accounts.environment_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_accounts.environment_id IS 'The environment in the vault this service account will recieve secrets from.';


--
-- Name: COLUMN service_accounts.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_accounts.name IS 'The name of this service account.';


--
-- Name: COLUMN service_accounts.encrypted_ecdh_private_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.service_accounts.encrypted_ecdh_private_key IS 'A wrapped ECDH private key, used to decrypt the secrets for this service account.';


--
-- Name: service_accounts_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.service_accounts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: service_accounts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.service_accounts_id_seq OWNED BY public.service_accounts.id;


--
-- Name: sessions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sessions (
    id integer NOT NULL,
    session_verifier character varying NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    otp_code_encrypted character varying NOT NULL,
    otp_code_attempts integer DEFAULT 0 NOT NULL,
    otp_code_confirmed boolean DEFAULT false NOT NULL,
    otp_code_sent boolean DEFAULT false NOT NULL
);


--
-- Name: TABLE sessions; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sessions IS 'The users login sessions';


--
-- Name: COLUMN sessions.session_verifier; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sessions.session_verifier IS ' The session is a 32 byte random number stored in their cookie. This is the sha256 hash of that value.';


--
-- Name: COLUMN sessions.otp_code_encrypted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sessions.otp_code_encrypted IS 'A 6 digit code that is encrypted here to prevent attackers with read access to the database being able to use it.';


--
-- Name: COLUMN sessions.otp_code_attempts; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sessions.otp_code_attempts IS 'We count OTP attempts to prevent brute forcing.';


--
-- Name: COLUMN sessions.otp_code_confirmed; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sessions.otp_code_confirmed IS 'Once the user enters the correct value this gets set to true.';


--
-- Name: COLUMN sessions.otp_code_sent; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sessions.otp_code_sent IS 'Have we sent the OTP code?';


--
-- Name: sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.sessions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.sessions_id_seq OWNED BY public.sessions.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id integer NOT NULL,
    email character varying NOT NULL,
    first_name character varying,
    last_name character varying,
    master_password_hash character varying NOT NULL,
    protected_symmetric_key character varying NOT NULL,
    protected_ecdsa_private_key character varying NOT NULL,
    ecdsa_public_key character varying NOT NULL,
    protected_ecdh_private_key character varying NOT NULL,
    ecdh_public_key character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.users IS 'Contains users and their private and public keys';


--
-- Name: COLUMN users.first_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.first_name IS 'The first name, not captured on registration for faster on boarding.';


--
-- Name: COLUMN users.last_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.last_name IS 'The last name, not captured on registration for faster on boarding.';


--
-- Name: COLUMN users.master_password_hash; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.master_password_hash IS 'Hash of the users master password for authentication';


--
-- Name: COLUMN users.protected_symmetric_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.protected_symmetric_key IS 'Wrapped AES-GCM key for symmetric encryption and decryption';


--
-- Name: COLUMN users.protected_ecdsa_private_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.protected_ecdsa_private_key IS 'Wrapped ECDSA key for signing';


--
-- Name: COLUMN users.ecdsa_public_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.ecdsa_public_key IS 'Public ECDSA key for signature verification';


--
-- Name: COLUMN users.protected_ecdh_private_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.protected_ecdh_private_key IS 'Wrapped ECDH key for public key encryption and key negotiation';


--
-- Name: COLUMN users.ecdh_public_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.ecdh_public_key IS 'Public ECDH key for public key encryption and key negotiation';


--
-- Name: users_environments; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users_environments (
    environment_id integer NOT NULL,
    user_id integer NOT NULL
);


--
-- Name: TABLE users_environments; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.users_environments IS 'When we add a user to a vault we can select which environemnts they are allowed to see.';


--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: users_vaults; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users_vaults (
    user_id integer NOT NULL,
    vault_id integer NOT NULL,
    ecdh_public_key character varying NOT NULL,
    encrypted_vault_key character varying NOT NULL
);


--
-- Name: TABLE users_vaults; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.users_vaults IS 'Connects users to vaults and holds a copy of the vault key encrypted with their AES key.';


--
-- Name: COLUMN users_vaults.ecdh_public_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users_vaults.ecdh_public_key IS 'An ECDH public key used to encrypt the vaults secrets for this user.';


--
-- Name: COLUMN users_vaults.encrypted_vault_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users_vaults.encrypted_vault_key IS 'A wrapped ECDH private key, used to decrypt the secrets for this user.';


--
-- Name: vaults; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.vaults (
    id integer NOT NULL,
    organisation_id integer NOT NULL,
    name character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: TABLE vaults; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.vaults IS 'Vaults allow users to divide secrets into logical groupings.';


--
-- Name: COLUMN vaults.organisation_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.vaults.organisation_id IS 'Vaults belong to an organisation';


--
-- Name: COLUMN vaults.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.vaults.name IS 'The user supplied name of the vault';


--
-- Name: vaults_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.vaults_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: vaults_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.vaults_id_seq OWNED BY public.vaults.id;


--
-- Name: audit_trail id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_trail ALTER COLUMN id SET DEFAULT nextval('public.audit_trail_id_seq'::regclass);


--
-- Name: environments id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.environments ALTER COLUMN id SET DEFAULT nextval('public.environments_id_seq'::regclass);


--
-- Name: invitations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invitations ALTER COLUMN id SET DEFAULT nextval('public.invitations_id_seq'::regclass);


--
-- Name: organisations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.organisations ALTER COLUMN id SET DEFAULT nextval('public.organisations_id_seq'::regclass);


--
-- Name: secrets id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.secrets ALTER COLUMN id SET DEFAULT nextval('public.secrets_id_seq'::regclass);


--
-- Name: service_account_secrets id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_account_secrets ALTER COLUMN id SET DEFAULT nextval('public.service_account_secrets_id_seq'::regclass);


--
-- Name: service_accounts id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_accounts ALTER COLUMN id SET DEFAULT nextval('public.service_accounts_id_seq'::regclass);


--
-- Name: sessions id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sessions ALTER COLUMN id SET DEFAULT nextval('public.sessions_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: vaults id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vaults ALTER COLUMN id SET DEFAULT nextval('public.vaults_id_seq'::regclass);


--
-- Name: audit_trail audit_trail_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_trail
    ADD CONSTRAINT audit_trail_pkey PRIMARY KEY (id);


--
-- Name: environments environments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.environments
    ADD CONSTRAINT environments_pkey PRIMARY KEY (id);


--
-- Name: invitations invitations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invitations
    ADD CONSTRAINT invitations_pkey PRIMARY KEY (id);


--
-- Name: organisation_users organisation_users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.organisation_users
    ADD CONSTRAINT organisation_users_pkey PRIMARY KEY (user_id, organisation_id);


--
-- Name: organisations organisations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.organisations
    ADD CONSTRAINT organisations_pkey PRIMARY KEY (id);


--
-- Name: roles_permissions roles_permissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.roles_permissions
    ADD CONSTRAINT roles_permissions_pkey PRIMARY KEY (role, permission);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: secrets secrets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.secrets
    ADD CONSTRAINT secrets_pkey PRIMARY KEY (id);


--
-- Name: service_account_secrets service_account_secrets_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_account_secrets
    ADD CONSTRAINT service_account_secrets_pkey PRIMARY KEY (id);


--
-- Name: service_accounts service_accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_accounts
    ADD CONSTRAINT service_accounts_pkey PRIMARY KEY (id);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users_environments users_environments_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users_environments
    ADD CONSTRAINT users_environments_pkey PRIMARY KEY (environment_id, user_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users_vaults users_vaults_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users_vaults
    ADD CONSTRAINT users_vaults_pkey PRIMARY KEY (user_id, vault_id);


--
-- Name: vaults vaults_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vaults
    ADD CONSTRAINT vaults_pkey PRIMARY KEY (id);


--
-- Name: secrets set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.secrets FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: service_account_secrets set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.service_account_secrets FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: service_accounts set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.service_accounts FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: users set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.users FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: vaults set_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_updated_at BEFORE UPDATE ON public.vaults FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


--
-- Name: users_environments fk_environment; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users_environments
    ADD CONSTRAINT fk_environment FOREIGN KEY (environment_id) REFERENCES public.environments(id) ON DELETE CASCADE;


--
-- Name: invitations fk_organisation; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.invitations
    ADD CONSTRAINT fk_organisation FOREIGN KEY (organisation_id) REFERENCES public.organisations(id);


--
-- Name: vaults fk_organisation; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.vaults
    ADD CONSTRAINT fk_organisation FOREIGN KEY (organisation_id) REFERENCES public.organisations(id) ON DELETE CASCADE;


--
-- Name: service_accounts fk_organisation; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_accounts
    ADD CONSTRAINT fk_organisation FOREIGN KEY (organisation_id) REFERENCES public.organisations(id) ON DELETE CASCADE;


--
-- Name: audit_trail fk_organisation; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_trail
    ADD CONSTRAINT fk_organisation FOREIGN KEY (organisation_id) REFERENCES public.organisations(id) ON DELETE CASCADE;


--
-- Name: service_account_secrets fk_service_account; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.service_account_secrets
    ADD CONSTRAINT fk_service_account FOREIGN KEY (service_account_id) REFERENCES public.service_accounts(id) ON DELETE CASCADE;


--
-- Name: users_vaults fk_user; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users_vaults
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: users_environments fk_user; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users_environments
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: secrets fk_vault; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.secrets
    ADD CONSTRAINT fk_vault FOREIGN KEY (vault_id) REFERENCES public.vaults(id) ON DELETE CASCADE;


--
-- Name: users_vaults fk_vault; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users_vaults
    ADD CONSTRAINT fk_vault FOREIGN KEY (vault_id) REFERENCES public.vaults(id) ON DELETE CASCADE;


--
-- Name: environments fk_vault; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.environments
    ADD CONSTRAINT fk_vault FOREIGN KEY (vault_id) REFERENCES public.vaults(id) ON DELETE CASCADE;


--
-- Name: audit_trail; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.audit_trail ENABLE ROW LEVEL SECURITY;

--
-- Name: sessions authentication_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY authentication_policy ON public.sessions TO authentication USING (true);


--
-- Name: users authentication_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY authentication_policy ON public.users TO authentication USING (true);


--
-- Name: environments; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.environments ENABLE ROW LEVEL SECURITY;

--
-- Name: invitations; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.invitations ENABLE ROW LEVEL SECURITY;

--
-- Name: audit_trail multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.audit_trail TO application USING ((organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)));


--
-- Name: environments multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.environments TO application USING ((vault_id IN ( SELECT users_vaults.vault_id
   FROM public.users_vaults
  WHERE (users_vaults.user_id = public.current_app_user())))) WITH CHECK ((vault_id IN ( SELECT users_vaults.vault_id
   FROM public.users_vaults
  WHERE (users_vaults.user_id = public.current_app_user()))));


--
-- Name: invitations multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.invitations TO application USING (((organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)) OR ((email)::text IN ( SELECT users.email
   FROM public.users
  WHERE (users.id = public.current_app_user()))))) WITH CHECK ((organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)));


--
-- Name: POLICY multi_tenancy_policy ON invitations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON POLICY multi_tenancy_policy ON public.invitations IS 'A users can access inviation from one of the orgs or if it has the same email address';


--
-- Name: organisations multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.organisations TO application USING (((id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)) OR (created_by_user_id = public.current_app_user())));


--
-- Name: POLICY multi_tenancy_policy ON organisations; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON POLICY multi_tenancy_policy ON public.organisations IS 'A user can see all the orgs they have created or been invited to.';


--
-- Name: secrets multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.secrets TO application USING ((vault_id IN ( SELECT users_vaults.vault_id
   FROM public.users_vaults)));


--
-- Name: service_account_secrets multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.service_account_secrets TO application USING ((service_account_id IN ( SELECT service_account_secrets.service_account_id
   FROM public.service_accounts
  WHERE (service_accounts.organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)))));


--
-- Name: service_accounts multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.service_accounts TO application USING ((organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)));


--
-- Name: users multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.users TO application USING ((id IN ( SELECT public.get_users_for_app_user() AS get_users_for_app_user)));


--
-- Name: POLICY multi_tenancy_policy ON users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON POLICY multi_tenancy_policy ON public.users IS 'A user can see all the users for orgs they have created or been invited to.';


--
-- Name: users_vaults multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.users_vaults TO application USING (((vault_id IN ( SELECT users_vaults.vault_id
   FROM public.vaults)) AND (user_id IN ( SELECT public.get_users_for_app_user() AS get_users_for_app_user))));


--
-- Name: vaults multi_tenancy_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy ON public.vaults TO application USING ((organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)));


--
-- Name: organisation_users multi_tenancy_policy_delete; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy_delete ON public.organisation_users FOR DELETE TO application USING ((organisation_id IN ( SELECT public.get_orgs_for_app_user() AS get_orgs_for_app_user)));


--
-- Name: organisation_users multi_tenancy_policy_insert; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy_insert ON public.organisation_users FOR INSERT TO application WITH CHECK (((organisation_id IN ( SELECT invitations.organisation_id
   FROM public.invitations)) OR (organisation_id IN ( SELECT public.get_orgs_app_user_created() AS get_orgs_app_user_created))));


--
-- Name: POLICY multi_tenancy_policy_insert ON organisation_users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON POLICY multi_tenancy_policy_insert ON public.organisation_users IS 'A user on connect users with orgs it created or where an invite exists.';


--
-- Name: organisation_users multi_tenancy_policy_select; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY multi_tenancy_policy_select ON public.organisation_users FOR SELECT TO application USING (true);


--
-- Name: POLICY multi_tenancy_policy_select ON organisation_users; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON POLICY multi_tenancy_policy_select ON public.organisation_users IS 'Only disconnect a user from an org if we have access to that org.';


--
-- Name: organisation_users; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.organisation_users ENABLE ROW LEVEL SECURITY;

--
-- Name: organisations; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.organisations ENABLE ROW LEVEL SECURITY;

--
-- Name: audit_trail readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.audit_trail FOR SELECT TO authentication USING (true);


--
-- Name: environments readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.environments FOR SELECT TO authentication USING (true);


--
-- Name: invitations readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.invitations FOR SELECT TO authentication USING (true);


--
-- Name: organisation_users readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.organisation_users FOR SELECT TO authentication USING (true);


--
-- Name: organisations readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.organisations FOR SELECT TO authentication USING (true);


--
-- Name: roles_permissions readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.roles_permissions FOR SELECT TO authentication USING (true);


--
-- Name: schema_migrations readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.schema_migrations FOR SELECT TO authentication USING (true);


--
-- Name: secrets readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.secrets FOR SELECT TO authentication USING (true);


--
-- Name: service_account_secrets readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.service_account_secrets FOR SELECT TO authentication USING (true);


--
-- Name: service_accounts readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.service_accounts FOR SELECT TO authentication USING (true);


--
-- Name: sessions readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.sessions FOR SELECT TO authentication USING (true);


--
-- Name: users readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.users FOR SELECT TO authentication USING (true);


--
-- Name: users_environments readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.users_environments FOR SELECT TO authentication USING (true);


--
-- Name: users_vaults readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.users_vaults FOR SELECT TO authentication USING (true);


--
-- Name: vaults readonly_policy; Type: POLICY; Schema: public; Owner: -
--

CREATE POLICY readonly_policy ON public.vaults FOR SELECT TO authentication USING (true);


--
-- Name: secrets; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.secrets ENABLE ROW LEVEL SECURITY;

--
-- Name: service_account_secrets; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.service_account_secrets ENABLE ROW LEVEL SECURITY;

--
-- Name: service_accounts; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.service_accounts ENABLE ROW LEVEL SECURITY;

--
-- Name: users; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.users ENABLE ROW LEVEL SECURITY;

--
-- Name: users_vaults; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.users_vaults ENABLE ROW LEVEL SECURITY;

--
-- Name: vaults; Type: ROW SECURITY; Schema: public; Owner: -
--

ALTER TABLE public.vaults ENABLE ROW LEVEL SECURITY;

--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20220410155201'),
    ('20220410155211'),
    ('20220410155233'),
    ('20220410155252'),
    ('20220410155319'),
    ('20220621094035'),
    ('20220728091159');
