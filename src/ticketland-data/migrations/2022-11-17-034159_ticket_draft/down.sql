-- This file should undo anything in `up.sql`

ALTER TABLE tickets
DROP COLUMN draft;
