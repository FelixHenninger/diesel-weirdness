// @generated automatically by Diesel CLI.

diesel::table! {
    meta_a (id) {
        id -> Integer,
        data -> Text,
    }
}

diesel::table! {
    meta_b (id) {
        id -> Integer,
        data -> Text,
    }
}

diesel::table! {
    nested (id) {
        id -> Integer,
        name -> Nullable<Text>,
        parent_id -> Nullable<Integer>,
        meta_a_id -> Nullable<Integer>,
        meta_b_id -> Nullable<Integer>,
    }
}

diesel::joinable!(nested -> meta_a (meta_a_id));
diesel::joinable!(nested -> meta_b (meta_b_id));

diesel::allow_tables_to_appear_in_same_query!(
    meta_a,
    meta_b,
    nested,
);
