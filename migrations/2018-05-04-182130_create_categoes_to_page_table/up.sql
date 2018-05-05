CREATE TABLE IF NOT EXISTS "categories_to_pages" (
    "page_id" TEXT,
    "category" TEXT,
    Primary Key("page_id", "category")
);