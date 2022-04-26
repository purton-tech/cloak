--! insert_invitation(organisation_id, email, invitation_selector, invitation_verifier_hash) 
INSERT INTO 
    invitations (organisation_id, email, invitation_selector, invitation_verifier_hash)
    VALUES($1, $2, $3, $4)

--! get_invitation(invitation_selector) { id, organisation_id, email, invitation_selector, invitation_verifier_hash, created_at, updated_at}
SELECT 
    id, 
    organisation_id, 
    email, 
    invitation_selector, 
    invitation_verifier_hash,
    created_at,
    updated_at
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

--! get_all(organisation_id) { id, email, invitation_selector, invitation_verifier_hash, organisation_id, updated_at, created_at} *
SELECT  
    id, 
    email,
    invitation_selector, 
    invitation_verifier_hash,
    organisation_id,
    updated_at, 
    created_at  
FROM 
    invitations 
WHERE organisation_id = $1