-- Your SQL goes here

ALTER TABLE events
DROP COLUMN location;
ALTER TABLE events
ADD location JSONB;
