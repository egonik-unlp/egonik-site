// @generated automatically by Diesel CLI.

diesel::table! {
    contact_informations (id) {
        id -> Int4,
        personal_information_id -> Int4,
        #[max_length = 255]
        github -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        instagram -> Varchar,
        #[max_length = 255]
        linked_in -> Varchar,
    }
}

diesel::table! {
    job_experiences (id) {
        id -> Int4,
        date_from -> Date,
        date_to -> Nullable<Date>,
        #[max_length = 255]
        job_title -> Varchar,
        #[max_length = 255]
        accomplishments -> Varchar,
        #[max_length = 255]
        responsabilities -> Varchar,
    }
}

diesel::table! {
    job_institutions (id) {
        id -> Int4,
        job_experience_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::table! {
    personal_informations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        surname -> Varchar,
        #[max_length = 255]
        image_url -> Varchar,
        birth_date -> Date,
    }
}

diesel::table! {
    portfolio_items (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        public -> Bool,
        #[max_length = 255]
        public_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    publication_items (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        abs -> Text,
        year -> Int4,
        #[max_length = 255]
        journal -> Varchar,
        #[max_length = 400]
        link -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        portfolio_item_id -> Int4,
        #[max_length = 255]
        value -> Varchar,
    }
}

diesel::joinable!(contact_informations -> personal_informations (personal_information_id));
diesel::joinable!(job_institutions -> job_experiences (job_experience_id));
diesel::joinable!(tags -> portfolio_items (portfolio_item_id));

diesel::allow_tables_to_appear_in_same_query!(
    contact_informations,
    job_experiences,
    job_institutions,
    personal_informations,
    portfolio_items,
    publication_items,
    tags,
);
