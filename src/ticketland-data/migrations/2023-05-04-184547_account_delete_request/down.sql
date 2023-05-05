-- This file should undo anything in `up.sql`

ALTER TABLE accounts
DROP COLUMN delete_request_at,
DROP COLUMN deleted_at;
