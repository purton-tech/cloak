--! insert_invitation(organisation_id, first_name, last_name, email, 
--!     invitation_selector, invitation_verifier_hash, roles) 
INSERT INTO 
    invitations (
        organisation_id, 
        email, 
        first_name, 
        last_name, 
        invitation_selector, 
        invitation_verifier_hash, 
        roles)
    VALUES($1, $2, $3, $4, $5, $6, $7)

--! get_invitation(invitation_selector) { 
--!     id, organisation_id, email, first_name, 
--!     last_name, invitation_selector, invitation_verifier_hash, roles, created_at}
SELECT 
    id, 
    organisation_id, 
    email, 
    first_name, 
    last_name, 
    invitation_selector, 
    invitation_verifier_hash,
    roles,
    created_at
FROM 
    invitations 
WHERE
    invitation_selector = $1

--! delete_invitation(email, organisation_id)
DELETE FROM
    invitations
WHERE
    email = $1
AND
    organisation_id = $2

--! get_all(organisation_id) { 
--!     id, email, first_name, last_name, invitation_selector, invitation_verifier_hash, 
--!     organisation_id, roles, created_at} *
SELECT  
    id, 
    email,
    first_name, 
    last_name, 
    invitation_selector, 
    invitation_verifier_hash,
    organisation_id,
    roles,
    created_at  
FROM 
    invitations 
WHERE organisation_id = $1