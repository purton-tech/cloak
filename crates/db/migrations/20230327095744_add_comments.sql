-- migrate:up
ALTER TYPE audit_action ADD VALUE 'RenameVault' AFTER 'DeleteVault';

-- migrate:down

