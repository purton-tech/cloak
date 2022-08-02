--! insert_invitation(organisation_id, email, first_name, last_name, invitation_selector, invitation_verifier_hash, roles) 
INSERT INTO 
    invitations (
        organisation_id, 
        email, 
        first_name, 
        last_name, 
        invitation_selector, 
        invitation_verifier_hash, 
        roles)
    VALUES(
        :organisation_id, 
        :email, 
        :first_name, 
        :last_name, 
        :invitation_selector, 
        :invitation_verifier_hash, 
        :roles);

--! get_invitation(invitation_selector)
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
    invitation_selector = :invitation_selector;

--! delete_invitation(email, organisation_id)
DELETE FROM
    invitations
WHERE
    email = :email
AND
    organisation_id = :organisation_id;

--! get_all(organisation_id)
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
WHERE organisation_id = :organisation_id;