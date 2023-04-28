-- This file should undo anything in `up.sql`
ALTER TABLE events
DROP COLUMN resale_cap,
DROP COLUMN organizer_resale_fee;
