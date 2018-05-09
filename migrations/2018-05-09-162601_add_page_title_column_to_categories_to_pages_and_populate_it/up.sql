ALTER TABLE categories_to_pages ADD COLUMN page_title TEXT;

update categories_to_pages 
SET 
    page_title = (SELECT title FROM conspiracies WHERE conspiracies.page_id = categories_to_pages.page_id)
WHERE EXISTS (
   select * from conspiracies where conspiracies.page_id =  categories_to_pages.page_id
);
