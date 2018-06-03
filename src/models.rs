use diesel::prelude::*;
use schema::{tags};

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