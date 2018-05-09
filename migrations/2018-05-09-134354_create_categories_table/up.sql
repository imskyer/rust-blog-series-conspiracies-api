CREATE TABLE IF NOT EXISTS "categories" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "category" TEXT NOT NULL UNIQUE,
    "raw_name" TEXT NOT NULL UNIQUE
);

insert into categories (category, raw_name) VALUES
('Conspiracy theories', 'Category:Conspiracy theories'),
('Fringe theory', 'Category:Fringe theory'),
('Pejoratives', 'Category:Pejoratives')