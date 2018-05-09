
update categories_to_pages 
SET 
    category = (SELECT categories.category FROM categories WHERE categories.raw_name = categories_to_pages.category),
    category_id = (SELECT categories.id FROM categories WHERE categories.raw_name = categories_to_pages.category)
WHERE EXISTS (
   select * from categories where categories.raw_name =  categories_to_pages.category
);