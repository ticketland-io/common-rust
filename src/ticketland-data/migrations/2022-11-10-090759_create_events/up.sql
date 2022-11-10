-- Your SQL goes here
CREATE TABLE accounts (
  uid VARCHAR PRIMARY KEY,
  created_at TIMESTAMP NOT NULL,
  mnemonic VARCHAR UNIQUE NOT NULL,
  pubkey VARCHAR UNIQUE NOT NULL,
  name VARCHAR,
  email VARCHAR UNIQUE,
  photo_url VARCHAR
);

CREATE TABLE canva_accounts (
  canva_uid VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE events (
  event_id VARCHAR PRIMARY KEY,
  created_at TIMESTAMP NOT NULL,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  location VARCHAR,
  venue VARCHAR,
  event_type INT NOT NULL,
  start_date TIMESTAMP NOT NULL,
  end_date TIMESTAMP NOT NULL,
  category INT NOT NULL,
  event_capacity VARCHAR(64) NOT NULL,
  file_type VARCHAR(10),
  arweave_tx_id VARCHAR,
  metadata_uploaded BOOL NOT NULL,
  image_uploaded BOOL NOT NULL
);

CREATE TABLE sales (
  id VARCHAR PRIMARY KEY,
  event_id VARCHAR NOT NULL REFERENCES events(event_id),
  created_at TIMESTAMP NOT NULL,
  account VARCHAR NOT NULL,
  ticket_type_index SMALLINT NOT NULL,
  ticket_type_name VARCHAR NOT NULL,
  n_tickets INT NOT NULL,
  sale_start_ts TIMESTAMP NOT NULL,
  sale_end_ts TIMESTAMP NOT NULL,
  sale_type JSONB NOT NULL
);

CREATE TABLE ticket_onchain_accounts (
  ticket_nft VARCHAR PRIMARY KEY,
  ticket_metadata VARCHAR UNIQUE NOT NULL
);

CREATE TABLE tickets (
  id SERIAL PRIMARY KEY,
  ticket_nft VARCHAR NOT NULL REFERENCES ticket_onchain_accounts(ticket_nft),
  event_id VARCHAR NOT NULL REFERENCES events(event_id),
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  created_at TIMESTAMP NOT NULL,
  ticket_type_index SMALLINT NOT NULL,
  seat_name VARCHAR NOT NULL,
  seat_index INT NOT NULL
);

CREATE TABLE sell_listings (
  id SERIAL PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  ticket_nft VARCHAR NOT NULL REFERENCES ticket_onchain_accounts(ticket_nft),
  created_at TIMESTAMP NOT NULL,
  ask_price BIGINT NOT NULL
);

CREATE TABLE buy_listings (
  id SERIAL PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  created_at TIMESTAMP NOT NULL,
  bid_price BIGINT NOT NULL
);

CREATE TABLE metadata (
  id SERIAL PRIMARY KEY,
  event_id VARCHAR NOT NULL REFERENCES events(event_id),
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  image VARCHAR NOT NULL
);

CREATE TABLE metadata_attributes (
  id SERIAL PRIMARY KEY,
  metadata_id INT NOT NULL REFERENCES metadata(id),
  trait_type VARCHAR NOT NULL,
  value VARCHAR NOT NULL
);

CREATE TABLE canva_designs (
  design_id VARCHAR PRIMARY KEY,
  canva_uid VARCHAR NOT NULL REFERENCES canva_accounts(canva_uid),
  created_at TIMESTAMP NOT NULL,
  url VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  file_type VARCHAR NOT NULL
);

CREATE TABLE stripe_accounts (
  stripe_uid VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  account_link VARCHAR,
  status SMALLINT NOT NULL
);

CREATE TABLE account_events (
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  event_id VARCHAR NOT NULL REFERENCES events(event_id),
  PRIMARY KEY(account_id, event_id)
);

CREATE TABLE seat_ranges (
  id SERIAL PRIMARY KEY,
  sale_id VARCHAR NOT NULL REFERENCES sales(id),
  l INT NOT NULL,
  r INT NOT NULL
);
  
CREATE TABLE api_clients (
  client_id VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid),
  created_at TIMESTAMP NOT NULL,
  client_secret VARCHAR NOT NULL
);
