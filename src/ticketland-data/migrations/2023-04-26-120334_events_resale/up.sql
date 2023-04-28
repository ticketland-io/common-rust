-- Your SQL goes here
-- The reason we use on update cascade on the account_id is
-- let's say that your primary key is a 10 digit UPC bar code and because of expansion
-- you need to change it to a 13-digit UPC bar code. In that case, ON UPDATE CASCADE would
-- allow you to change the primary key value and any tables that have foreign key references to the value
-- will be changed accordingly.
ALTER TABLE events
ADD resale_cap SMALLINT NOT NULL DEFAULT 0,
ADD organizer_resale_fee SMALLINT NOT NULL DEFAULT 0;
