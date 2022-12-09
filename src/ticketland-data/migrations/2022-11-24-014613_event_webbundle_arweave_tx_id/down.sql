-- This file should undo anything in `up.sql`

ALTER TABLE events
DROP COLUMN webbundle_arweave_tx_id;
