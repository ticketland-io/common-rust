// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        uid -> Varchar,
        mnemonic -> Varchar,
        pubkey -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        photo_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    canva_accounts (id) {
        id -> Int4,
        account_id -> Nullable<Int4>,
    }
}

diesel::joinable!(canva_accounts -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    canva_accounts,
);
