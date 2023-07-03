-- Your SQL goes here


-- Delete all previous tables that are related to solana accounts
-- DROP TABLE ticket_onchain_accounts CASCADE;
-- DROP TABLE tickets CASCADE;
-- DROP TABLE sell_listings CASCADE;
-- DROP TABLE buy_listings CASCADE;
-- DROP TABLE metadata_attributes CASCADE;
-- DROP TABLE metadata CASCADE;
-- DROP TABLE seat_ranges CASCADE;
-- DROP TABLE sales CASCADE;
-- DROP TABLE events CASCADE;
-- DROP TABLE ticket_images CASCADE;

CREATE TABLE events (
  event_id VARCHAR PRIMARY KEY,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  location JSONB,
  venue VARCHAR NOT NULL,
  event_type SMALLINT NOT NULL,
  visibility SMALLINT NOT NULL,
  start_date TIMESTAMP WITH TIME ZONE NOT NULL,
  end_date TIMESTAMP WITH TIME ZONE NOT NULL,
  category SMALLINT NOT NULL,
  event_sui_address VARCHAR(66) UNIQUE,
  organizer_cap VARCHAR(66) UNIQUE,
  operator_cap VARCHAR(66) UNIQUE,
  event_nft VARCHAR(66) UNIQUE,
  event_capacity_bitmap_address VARCHAR(66) UNIQUE,
  webbundle_arweave_tx_id VARCHAR UNIQUE,
  draft BOOL NOT NULL
);

CREATE TABLE ticket_types (
  ticket_type_sui_address VARCHAR(66) UNIQUE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  ticket_type_index SMALLINT NOT NULL,
  ticket_type_name VARCHAR NOT NULL,
  n_tickets INT NOT NULL,
  sale_start_ts TIMESTAMP WITH TIME ZONE NOT NULL,
  sale_end_ts TIMESTAMP WITH TIME ZONE NOT NULL,
  sale_type JSONB NOT NULL,
  PRIMARY KEY(event_id, ticket_type_index)
);

CREATE TABLE seat_ranges (
  event_id VARCHAR NOT NULL,
  ticket_type_index SMALLINT NOT NULL,
  l INT NOT NULL,
  r INT NOT NULL,
  PRIMARY KEY(event_id, ticket_type_index, l, r),
  FOREIGN KEY(event_id, ticket_type_index) REFERENCES ticket_types(event_id, ticket_type_index) ON DELETE CASCADE
);

CREATE TABLE cnts (
  cnt_sui_address VARCHAR(66) UNIQUE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  ticket_type_index SMALLINT NOT NULL,
  seat_name VARCHAR NOT NULL,
  seat_index INT NOT NULL,
  attended BOOL DEFAULT false NOT NULL,
  draft BOOL NOT NULL,
  PRIMARY KEY(event_id, seat_index)
);

CREATE TABLE nft_details (
  -- id SERIAL PRIMARY KEY,
  nft_name VARCHAR NOT NULL,
  nft_description TEXT NOT NULL,
  content_type VARCHAR NOT NULL,
  arweave_tx_id VARCHAR PRIMARY KEY
);

CREATE TABLE event_nft_details (
  ref_name VARCHAR PRIMARY KEY,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  nft_details_id VARCHAR NOT NULL REFERENCES nft_details(arweave_tx_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE ticket_type_nft_details (
  ref_name VARCHAR PRIMARY KEY,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  ticket_type_index SMALLINT NOT NULL,
  nft_details_id VARCHAR NOT NULL REFERENCES nft_details(arweave_tx_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE properties (
  id SERIAL PRIMARY KEY,
  nft_details_id VARCHAR NOT NULL REFERENCES nft_details(arweave_tx_id) ON DELETE CASCADE ON UPDATE CASCADE,
  trait_type VARCHAR NOT NULL,
  value VARCHAR NOT NULL,
  UNIQUE(nft_details_id, trait_type, value)
);

CREATE TABLE event_nfts (
  event_nft_sui_address VARCHAR(66) UNIQUE,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  ref_name VARCHAR PRIMARY KEY REFERENCES event_nft_details(ref_name)  ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE ticket_type_nfts (
  ticket_type_nft_sui_address VARCHAR(66) UNIQUE,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  ref_name VARCHAR PRIMARY KEY REFERENCES ticket_type_nft_details(ref_name)  ON DELETE CASCADE ON UPDATE CASCADE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  ticket_type_index SMALLINT NOT NULL,
  FOREIGN KEY(event_id, ticket_type_index) REFERENCES ticket_types(event_id, ticket_type_index) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE listings (
  listing_id VARCHAR PRIMARY KEY,
  listing_sui_address VARCHAR(66) UNIQUE,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  cnt_sui_address VARCHAR(66) NOT NULL REFERENCES cnts(cnt_sui_address),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  ask_price BIGINT NOT NULL,
  is_open BOOL DEFAULT true NOT NULL,
  closed_at TIMESTAMP WITH TIME ZONE,
  draft BOOL NOT NULL
);

CREATE TABLE offers (
  offer_id VARCHAR PRIMARY KEY,
  offer_sui_address VARCHAR(66) UNIQUE,
  account_id VARCHAR NOT NULL REFERENCES accounts(uid) ON DELETE CASCADE ON UPDATE CASCADE,
  event_id VARCHAR NOT NULL REFERENCES events(event_id) ON DELETE CASCADE ON UPDATE CASCADE,
  ticket_type_index SMALLINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  bid_price BIGINT NOT NULL,
  is_open BOOL DEFAULT true NOT NULL,
  closed_at TIMESTAMP WITH TIME ZONE,
  draft BOOL NOT NULL,
  FOREIGN KEY(event_id, ticket_type_index) REFERENCES ticket_types(event_id, ticket_type_index) ON DELETE CASCADE ON UPDATE CASCADE
);
