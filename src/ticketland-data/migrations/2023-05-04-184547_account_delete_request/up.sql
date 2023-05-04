-- Your SQL goes here

ALTER TABLE accounts
ADD delete_request_at TIMESTAMP WITH TIME ZONE DEFAULT NULL;
