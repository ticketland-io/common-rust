-- Your SQL goes here
-- The reason we use on update cascade on the account_id is
-- let's say that your primary key is a 10 digit UPC bar code and because of expansion
-- you need to change it to a 13-digit UPC bar code. In that case, ON UPDATE CASCADE would
-- allow you to change the primary key value and any tables that have foreign key references to the value
-- will be changed accordingly.
CREATE TABLE accounts (
  uid VARCHAR PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  mnemonic VARCHAR UNIQUE NOT NULL,
  pubkey VARCHAR UNIQUE NOT NULL,
  name VARCHAR,
  email VARCHAR UNIQUE,
  photo_url VARCHAR
);

CREATE TABLE canva_accounts (
  canva_uid VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE events (
  event_id VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  location JSONB,
  venue VARCHAR,
  event_type SMALLINT NOT NULL,
  visibility SMALLINT NOT NULL,
  start_date TIMESTAMP WITH TIME ZONE NOT NULL,
  end_date TIMESTAMP WITH TIME ZONE NOT NULL,
  category SMALLINT NOT NULL,
  event_capacity VARCHAR(64) NOT NULL,
  file_type VARCHAR(10),
  arweave_tx_id VARCHAR,
  webbundle_arweave_tx_id VARCHAR,
  image_uploaded BOOL NOT NULL,
  draft BOOL NOT NULL
);

CREATE TABLE sales (
  account VARCHAR PRIMARY KEY,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  ticket_type_index SMALLINT NOT NULL,
  ticket_type_name VARCHAR NOT NULL,
  n_tickets INT NOT NULL,
  sale_start_ts TIMESTAMP WITH TIME ZONE NOT NULL,
  sale_end_ts TIMESTAMP WITH TIME ZONE NOT NULL,
  sale_type JSONB NOT NULL
);

CREATE TABLE ticket_onchain_accounts (
  ticket_nft VARCHAR PRIMARY KEY,
  ticket_metadata VARCHAR UNIQUE NOT NULL
);

CREATE TABLE tickets (
  ticket_nft VARCHAR PRIMARY KEY REFERENCES ticket_onchain_accounts(ticket_nft),
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  ticket_type_index SMALLINT NOT NULL,
  seat_name VARCHAR NOT NULL,
  seat_index INT NOT NULL,
  attended BOOL DEFAULT false NOT NULL,
  draft BOOL NOT NULL
);

CREATE TABLE sell_listings (
  sol_account VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  ticket_nft VARCHAR NOT NULL REFERENCES ticket_onchain_accounts(ticket_nft) ON DELETE CASCADE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  ask_price BIGINT NOT NULL,
  is_open BOOL DEFAULT true NOT NULL,
  closed_at TIMESTAMP WITH TIME ZONE,
  draft BOOL NOT NULL
);

CREATE TABLE buy_listings (
  sol_account VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  bid_price BIGINT NOT NULL,
  is_open BOOL DEFAULT true NOT NULL,
  closed_at TIMESTAMP WITH TIME ZONE,
  n_listing BIGINT DEFAULT 0 NOT NULL,
  draft BOOL NOT NULL
);

CREATE TABLE metadata (
  id SERIAL PRIMARY KEY,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  image VARCHAR NOT NULL
);

CREATE TABLE metadata_attributes (
  id SERIAL PRIMARY KEY,
  metadata_id INT NOT NULL REFERENCES metadata(id) ON DELETE CASCADE,
  trait_type VARCHAR NOT NULL,
  value VARCHAR NOT NULL
);

CREATE TABLE canva_designs (
  design_id VARCHAR PRIMARY KEY,
  canva_uid VARCHAR NOT NULL REFERENCES canva_accounts(canva_uid) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  url VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  file_type VARCHAR NOT NULL
);

CREATE TABLE stripe_accounts (
  stripe_uid VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  account_link VARCHAR,
  status SMALLINT NOT NULL
);

CREATE TABLE stripe_customers (
  customer_uid VARCHAR PRIMARY KEY,
  stripe_uid VARCHAR NOT NULL REFERENCES stripe_accounts(stripe_uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);


CREATE TABLE seat_ranges (
  sale_account VARCHAR NOT NULL REFERENCES sales(account) ON DELETE CASCADE,
  l INT NOT NULL,
  r INT NOT NULL,
  PRIMARY KEY(sale_account, l, r)
);

CREATE TABLE api_clients (
  client_id VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  client_secret VARCHAR NOT NULL
);
