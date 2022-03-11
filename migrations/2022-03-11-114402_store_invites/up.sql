CREATE TABLE invitations (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    email VARCHAR NOT NULL,
    invitation VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
   CONSTRAINT fk_organisation
      FOREIGN KEY(organisation_id) 
	  REFERENCES organisations(id)
);

GRANT SELECT, INSERT, UPDATE, DELETE ON invitations TO cloak;
GRANT USAGE, SELECT ON invitations_id_seq TO cloak;