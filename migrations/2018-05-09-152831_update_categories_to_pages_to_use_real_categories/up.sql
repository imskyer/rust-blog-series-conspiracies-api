ALTER TABLE categories_to_pages ADD COLUMN category_id int;


-- removes the wikipedia specific categories that aren't helpful for what I'm doing
DELETE FROM categories_to_pages WHERE category NOT IN (SELECT raw_name FROM categories); 
