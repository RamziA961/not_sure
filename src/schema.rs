// @generated automatically by Diesel CLI.

diesel::table! {
    individuals (id) {
        id -> Int4,
        #[max_length = 64]
        last_name -> Varchar,
        dob -> Date,
        fk_user -> Int4,
        #[max_length = 32]
        first_name -> Varchar,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        founded -> Date,
        fk_user -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        display_name -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        #[max_length = 20]
        handle -> Varchar,
        created_on -> Nullable<Timestamp>,
        #[max_length = 64]
        profile_uri -> Nullable<Varchar>,
        #[max_length = 128]
        photo_uri -> Nullable<Varchar>,
    }
}

diesel::joinable!(individuals -> users (fk_user));
diesel::joinable!(organizations -> users (fk_user));

diesel::allow_tables_to_appear_in_same_query!(individuals, organizations, users,);
