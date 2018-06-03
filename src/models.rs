use schema::{conspiracy_tags, conspiracies, links_processed, tags};

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
#[table_name="conspiracy_tags"]
pub struct ConspiracyTag {
    pub conspiracy_id: String,
    pub conspiracy_title: String,      
    pub tag_id: i32,
    pub tag_name: String
}


impl ConspiracyTag {
    pub fn new(conspiracy_id: String, conspiracy_title: String, tag_id: i32, tag_name: String) -> ConspiracyTag {
        ConspiracyTag {
            conspiracy_id: conspiracy_id,
            conspiracy_title: conspiracy_title,
            tag_id: tag_id,
            tag_name: tag_name,
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
