-- Your SQL goes here


CREATE TABLE stripe_customers (
  customer_uid VARCHAR PRIMARY KEY,
  stripe_uid VARCHAR NOT NULL REFERENCES stripe_accounts(stripe_uid) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
