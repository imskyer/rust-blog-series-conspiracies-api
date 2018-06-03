CREATE TABLE IF NOT EXISTS "conspiracy_tags" (
    "conspiracy_id" TEXT,
    "tag_id" int,
    "conspiracy_title" TEXT,
    "tag_name" TEXT, 
    Primary Key("conspiracy_id", "tag_id"),
    FOREIGN KEY("conspiracy_id") REFERENCES conspiracies("page_id"),
    FOREIGN KEY("tag_id") REFERENCES tags("id")
);

DROP TABLE "categories_to_pages";