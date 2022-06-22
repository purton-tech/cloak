--! audit() { user_id, created_at, action, entity } *
SELECT user_id, created_at, action, entity FROM audit_trail;

--! insert(user_id, action, entity)
INSERT INTO audit_trail (user_id, action, entity) VALUES ($1, $2, $3)