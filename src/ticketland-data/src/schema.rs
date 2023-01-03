// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (uid) {
        uid -> Varchar,
        created_at -> Nullable<Timestamptz>,
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
        created_at -> Nullable<Timestamptz>,
        client_secret -> Varchar,
    }
}

diesel::table! {
    buy_listings (id) {
        id -> Int4,
        account_id -> Varchar,
        event_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        sol_account -> Varchar,
        bid_price -> Int8,
        is_open -> Bool,
        n_listing -> Int8,
        draft -> Bool,
        closed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    canva_accounts (canva_uid) {
        canva_uid -> Varchar,
        account_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    canva_designs (design_id) {
        design_id -> Varchar,
        canva_uid -> Varchar,
        created_at -> Nullable<Timestamptz>,
        url -> Varchar,
        name -> Varchar,
        file_type -> Varchar,
    }
}

diesel::table! {
    events (event_id) {
        event_id -> Varchar,
        account_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        name -> Varchar,
        description -> Text,
        location -> Nullable<Jsonb>,
        venue -> Nullable<Varchar>,
        event_type -> Int2,
        visibility -> Int2,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        category -> Int2,
        event_capacity -> Varchar,
        file_type -> Nullable<Varchar>,
        arweave_tx_id -> Nullable<Varchar>,
        image_uploaded -> Bool,
        draft -> Bool,
        webbundle_arweave_tx_id -> Nullable<Varchar>,
        location -> Nullable<Jsonb>,
    }
}

diesel::table! {
    metadata (id) {
        id -> Int4,
        event_id -> Varchar,
        name -> Varchar,
        description -> Text,
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
    sales (account) {
        account -> Varchar,
        event_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        ticket_type_index -> Int2,
        ticket_type_name -> Varchar,
        n_tickets -> Int4,
        sale_start_ts -> Timestamptz,
        sale_end_ts -> Timestamptz,
        sale_type -> Jsonb,
    }
}

diesel::table! {
    seat_ranges (sale_account, l, r) {
        sale_account -> Varchar,
        l -> Int4,
        r -> Int4,
    }
}

diesel::table! {
    sell_listings (id) {
        id -> Int4,
        account_id -> Varchar,
        ticket_nft -> Varchar,
        event_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        sol_account -> Varchar,
        ask_price -> Int8,
        is_open -> Bool,
        draft -> Bool,
        closed_at -> Nullable<Timestamptz>,
        closed_at2 -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    stripe_accounts (stripe_uid) {
        stripe_uid -> Varchar,
        account_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        account_link -> Nullable<Varchar>,
        status -> Int2,
    }
}

diesel::table! {
    stripe_customers (customer_uid) {
        customer_uid -> Varchar,
        stripe_uid -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    ticket_onchain_accounts (ticket_nft) {
        ticket_nft -> Varchar,
        ticket_metadata -> Varchar,
    }
}

diesel::table! {
    tickets (ticket_nft) {
        ticket_nft -> Varchar,
        event_id -> Varchar,
        account_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
        ticket_type_index -> Int2,
        seat_name -> Varchar,
        seat_index -> Int4,
        attended -> Bool,
        draft -> Bool,
    }
}

diesel::joinable!(api_clients -> accounts (account_id));
diesel::joinable!(buy_listings -> accounts (account_id));
diesel::joinable!(buy_listings -> events (event_id));
diesel::joinable!(canva_accounts -> accounts (account_id));
diesel::joinable!(canva_designs -> canva_accounts (canva_uid));
diesel::joinable!(events -> accounts (account_id));
diesel::joinable!(metadata -> events (event_id));
diesel::joinable!(metadata_attributes -> metadata (metadata_id));
diesel::joinable!(sales -> events (event_id));
diesel::joinable!(seat_ranges -> sales (sale_account));
diesel::joinable!(sell_listings -> accounts (account_id));
diesel::joinable!(sell_listings -> events (event_id));
diesel::joinable!(sell_listings -> ticket_onchain_accounts (ticket_nft));
diesel::joinable!(stripe_accounts -> accounts (account_id));
diesel::joinable!(stripe_customers -> stripe_accounts (stripe_uid));
diesel::joinable!(tickets -> accounts (account_id));
diesel::joinable!(tickets -> events (event_id));
diesel::joinable!(tickets -> ticket_onchain_accounts (ticket_nft));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    api_clients,
    buy_listings,
    canva_accounts,
    canva_designs,
    events,
    metadata,
    metadata_attributes,
    sales,
    seat_ranges,
    sell_listings,
    stripe_accounts,
    stripe_customers,
    ticket_onchain_accounts,
    tickets,
);
