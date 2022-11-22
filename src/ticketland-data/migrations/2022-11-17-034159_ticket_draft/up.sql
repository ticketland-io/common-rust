-- Your SQL goes here

ALTER TABLE tickets
ADD draft BOOLEAN DEFAULT false NOT NULL;
