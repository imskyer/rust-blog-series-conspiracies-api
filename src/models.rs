use schema::{categories_to_pages, conspiracies, links_processed, tags};

#[derive(Insertable, Queryable, Debug)]
#[table_name="links_processed"]
pub struct LinkProcessed  {
    pub title: String, 
    pub processed: i32,
}

#[derive(Insertable, Queryable, Debug, Serialize)]
#[table_name="tags"]
pub struct Tag {
    id: i32,
    pub name: String,
    pub approved: i32
}

#[derive(Insertable, Debug)]
#[table_name="categories_to_pages"]
pub struct CategoryToPage {
    pub page_id: String,
    pub category: String,      
}


impl CategoryToPage {
    pub fn new(page_id: &str, category: String) -> CategoryToPage {
        CategoryToPage {
            page_id: page_id.to_string(),
            category: category,
        }
    }
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name="tags"]
pub struct NewTag {
    pub name: String
}

impl NewTag {
    pub fn new_tag(name: String) -> NewTag {
        NewTag{
            name: name
        }
    }
}

// I've added the derive debug so I can use println! to print it out
#[derive(Insertable, Debug, Queryable, Serialize)]
#[table_name="conspiracies"]
pub struct Conspiracy  {
    pub title: String, 
    pub page_id: String,
    summary: String,
    content: String,
    background: String, 
}

impl Conspiracy {
    pub fn new(title: String, page_id: String, summary: String, content: String, background: String) -> Conspiracy {
        Conspiracy{
            title: title,
            page_id: page_id,
            summary: summary,
            content: content,
            background: background,
        }
    }
}
