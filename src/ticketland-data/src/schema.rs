// @generated automatically by Diesel CLI.

diesel::table! {
    account_designs (design_id) {
        design_id -> Varchar,
        account_id -> Varchar,
        created_at -> Timestamp,
        url -> Varchar,
        name -> Varchar,
        file_type -> Varchar,
    }
}

diesel::table! {
    account_events (account_id, event_id) {
        account_id -> Varchar,
        event_id -> Varchar,
    }
}

diesel::table! {
    accounts (uid) {
        uid -> Varchar,
        created_at -> Timestamp,
        mnemonic -> Varchar,
        pubkey -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        photo_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    api_clients (client_id) {
        client_id -> Varchar,
        account_id -> Varchar,
        created_at -> Timestamp,
        client_secret -> Varchar,
    }
}

diesel::table! {
    buy_listings (id) {
        id -> Int4,
        created_at -> Timestamp,
        buyer_pub_key -> Varchar,
        bid_price -> Int4,
    }
}

diesel::table! {
    canva_accounts (canva_uid) {
        canva_uid -> Varchar,
        account_id -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    events (event_id) {
        event_id -> Varchar,
        created_at -> Timestamp,
        name -> Varchar,
        description -> Varchar,
        location -> Nullable<Varchar>,
        venue -> Nullable<Varchar>,
        event_type -> Int4,
        start_date -> Timestamp,
        end_date -> Timestamp,
        category -> Int4,
        event_capacity -> Varchar,
        file_type -> Nullable<Varchar>,
        arweave_tx_id -> Nullable<Varchar>,
        metadata_uploaded -> Bool,
        image_uploaded -> Bool,
    }
}

diesel::table! {
    metadata (id) {
        id -> Int4,
        event_id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        image -> Varchar,
    }
}

diesel::table! {
    metadata_attributes (id) {
        id -> Int4,
        metadata_id -> Int4,
        trait_type -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    sales (id) {
        id -> Int4,
        event_id -> Varchar,
        created_at -> Timestamp,
        ticket_type_index -> Int2,
        ticket_type_name -> Varchar,
        n_tickets -> Int4,
        sale_start_ts -> Timestamp,
        sale_end_ts -> Timestamp,
        sale_type -> Nullable<Jsonb>,
    }
}

diesel::table! {
    seat_ranges (id) {
        id -> Int4,
        sale_id -> Int4,
        l -> Int4,
        r -> Int4,
    }
}

diesel::table! {
    sell_listings (id) {
        id -> Int4,
        account_id -> Varchar,
        ticket_nft -> Varchar,
        created_at -> Timestamp,
        ask_price -> Int4,
    }
}

diesel::table! {
    stripe_accounts (stripe_uid) {
        stripe_uid -> Varchar,
        account_id -> Varchar,
        account_link -> Nullable<Varchar>,
        status -> Int2,
    }
}

diesel::table! {
    ticket_onchain_accounts (ticket_nft) {
        ticket_nft -> Varchar,
        ticket_metadata -> Varchar,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        ticket_nft -> Varchar,
        event_id -> Varchar,
        account_id -> Varchar,
        created_at -> Timestamp,
        ticket_type_index -> Int2,
        seat_name -> Varchar,
        seat_index -> Int4,
    }
}

diesel::joinable!(account_designs -> accounts (account_id));
diesel::joinable!(account_events -> accounts (account_id));
diesel::joinable!(account_events -> events (event_id));
diesel::joinable!(api_clients -> accounts (account_id));
diesel::joinable!(canva_accounts -> accounts (account_id));
diesel::joinable!(metadata -> events (event_id));
diesel::joinable!(metadata_attributes -> metadata (metadata_id));
diesel::joinable!(sales -> events (event_id));
diesel::joinable!(seat_ranges -> sales (sale_id));
diesel::joinable!(sell_listings -> accounts (account_id));
diesel::joinable!(sell_listings -> ticket_onchain_accounts (ticket_nft));
diesel::joinable!(stripe_accounts -> accounts (account_id));
diesel::joinable!(tickets -> accounts (account_id));
diesel::joinable!(tickets -> events (event_id));
diesel::joinable!(tickets -> ticket_onchain_accounts (ticket_nft));

diesel::allow_tables_to_appear_in_same_query!(
    account_designs,
    account_events,
    accounts,
    api_clients,
    buy_listings,
    canva_accounts,
    events,
    metadata,
    metadata_attributes,
    sales,
    seat_ranges,
    sell_listings,
    stripe_accounts,
    ticket_onchain_accounts,
    tickets,
);