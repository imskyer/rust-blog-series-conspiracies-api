table! {
    categories (id) {
        id -> Nullable<Integer>,
        category -> Text,
        raw_name -> Text,
    }
}

table! {
    conspiracies (page_id) {
        title -> Text,
        page_id -> Text,
        summary -> Text,
        content -> Text,
        background -> Text,
    }
}

table! {
    conspiracy_tags (conspiracy_id, tag_id) {
        conspiracy_id -> Nullable<Text>,
        tag_id -> Nullable<Integer>,
        conspiracy_title -> Nullable<Text>,
        tag_name -> Nullable<Text>,
    }
}

table! {
    links_processed (title) {
        title -> Nullable<Text>,
        processed -> Nullable<Integer>,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        approved -> Integer,
    }
}

joinable!(conspiracy_tags -> conspiracies (conspiracy_id));
joinable!(conspiracy_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    categories,
    conspiracies,
    conspiracy_tags,
    links_processed,
    tags,
);
