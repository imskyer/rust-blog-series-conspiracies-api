table! {
    categories (id) {
        id -> Integer,
        category -> Text,
        raw_name -> Text,
    }
}

table! {
    categories_to_pages (page_id, category) {
        page_id -> Text,
        category -> Text,
        category_id -> Integer,
        page_title -> Text,
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
        title -> Text,
        processed -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        approved -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    categories_to_pages,
    conspiracies,
    links_processed,
    tags,
);
