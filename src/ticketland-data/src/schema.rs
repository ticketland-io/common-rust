// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        uid -> Varchar,
        mnemonic -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        photo_url -> Nullable<Varchar>,
    }
}
