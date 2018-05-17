table! {
    categories_to_pages (page_id, category) {
        page_id -> Nullable<Text>,
        category -> Nullable<Text>,
    }
}

table! {
    conspiracies (page_id) {
        page_id -> Text,
        title -> Text,
        summary -> Text,
        content -> Text,
        background -> Text,
    }
}

table! {
    links_processed (title) {
        title -> Nullable<Text>,
        processed -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    categories_to_pages,
    conspiracies,
    links_processed,
);
