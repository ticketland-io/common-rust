-- Your SQL goes here
CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  uid VARCHAR NOT NULL,
  mnemonic VARCHAR NOT NULL,
  pubkey VARCHAR NOT NULL,
  name VARCHAR,
  email VARCHAR,
  photo_url VARCHAR
)
