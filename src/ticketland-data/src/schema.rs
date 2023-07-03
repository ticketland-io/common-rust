// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (uid) {
        uid -> Varchar,
        created_at -> Nullable<Timestamptz>,
        dapp_share -> Varchar,
        pubkey -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        photo_url -> Nullable<Varchar>,
        delete_request_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
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
    cnts (event_id, seat_index) {
        cnt_sui_address -> Nullable<Varchar>,
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

diesel::table! {
    event_nft_details (ref_name) {
        ref_name -> Varchar,
        event_id -> Varchar,
        nft_details_id -> Varchar,
    }
}

diesel::table! {
    event_nfts (ref_name) {
        event_nft_sui_address -> Nullable<Varchar>,
        account_id -> Varchar,
        event_id -> Varchar,
        ref_name -> Varchar,
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
        venue -> Varchar,
        event_type -> Int2,
        visibility -> Int2,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        category -> Int2,
        event_sui_address -> Nullable<Varchar>,
        organizer_cap -> Nullable<Varchar>,
        operator_cap -> Nullable<Varchar>,
        event_nft -> Nullable<Varchar>,
        event_capacity_bitmap_address -> Nullable<Varchar>,
        webbundle_arweave_tx_id -> Nullable<Varchar>,
        draft -> Bool,
    }
}

diesel::table! {
    listings (listing_id) {
        listing_id -> Varchar,
        listing_sui_address -> Nullable<Varchar>,
        account_id -> Varchar,
        event_id -> Varchar,
        cnt_sui_address -> Varchar,
        created_at -> Nullable<Timestamptz>,
        ask_price -> Int8,
        is_open -> Bool,
        closed_at -> Nullable<Timestamptz>,
        draft -> Bool,
    }
}

diesel::table! {
    nft_details (arweave_tx_id) {
        nft_name -> Varchar,
        nft_description -> Text,
        content_type -> Varchar,
        arweave_tx_id -> Varchar,
    }
}

diesel::table! {
    offers (offer_id) {
        offer_id -> Varchar,
        offer_sui_address -> Nullable<Varchar>,
        account_id -> Varchar,
        event_id -> Varchar,
        ticket_type_index -> Int2,
        created_at -> Nullable<Timestamptz>,
        bid_price -> Int8,
        is_open -> Bool,
        closed_at -> Nullable<Timestamptz>,
        draft -> Bool,
    }
}

diesel::table! {
    properties (id) {
        id -> Int4,
        nft_details_id -> Varchar,
        trait_type -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    seat_ranges (event_id, ticket_type_index, l, r) {
        event_id -> Varchar,
        ticket_type_index -> Int2,
        l -> Int4,
        r -> Int4,
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
        account_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    ticket_type_nft_details (ref_name) {
        ref_name -> Varchar,
        event_id -> Varchar,
        ticket_type_index -> Int2,
        nft_details_id -> Varchar,
    }
}

diesel::table! {
    ticket_type_nfts (ref_name) {
        ticket_type_nft_sui_address -> Nullable<Varchar>,
        account_id -> Varchar,
        ref_name -> Varchar,
        event_id -> Varchar,
        ticket_type_index -> Int2,
    }
}

diesel::table! {
    ticket_types (event_id, ticket_type_index) {
        ticket_type_sui_address -> Nullable<Varchar>,
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

diesel::joinable!(api_clients -> accounts (account_id));
diesel::joinable!(canva_accounts -> accounts (account_id));
diesel::joinable!(canva_designs -> canva_accounts (canva_uid));
diesel::joinable!(cnts -> accounts (account_id));
diesel::joinable!(cnts -> events (event_id));
diesel::joinable!(event_nft_details -> events (event_id));
diesel::joinable!(event_nft_details -> nft_details (nft_details_id));
diesel::joinable!(event_nfts -> accounts (account_id));
diesel::joinable!(event_nfts -> event_nft_details (ref_name));
diesel::joinable!(event_nfts -> events (event_id));
diesel::joinable!(events -> accounts (account_id));
diesel::joinable!(listings -> accounts (account_id));
diesel::joinable!(listings -> events (event_id));
diesel::joinable!(offers -> accounts (account_id));
diesel::joinable!(offers -> events (event_id));
diesel::joinable!(properties -> nft_details (nft_details_id));
diesel::joinable!(stripe_accounts -> accounts (account_id));
diesel::joinable!(stripe_customers -> accounts (account_id));
diesel::joinable!(ticket_type_nft_details -> events (event_id));
diesel::joinable!(ticket_type_nft_details -> nft_details (nft_details_id));
diesel::joinable!(ticket_type_nfts -> accounts (account_id));
diesel::joinable!(ticket_type_nfts -> events (event_id));
diesel::joinable!(ticket_type_nfts -> ticket_type_nft_details (ref_name));
diesel::joinable!(ticket_types -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    api_clients,
    canva_accounts,
    canva_designs,
    cnts,
    event_nft_details,
    event_nfts,
    events,
    listings,
    nft_details,
    offers,
    properties,
    seat_ranges,
    stripe_accounts,
    stripe_customers,
    ticket_type_nft_details,
    ticket_type_nfts,
    ticket_types,
);
