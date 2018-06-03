use actix::prelude::*;
use wiki::{WikiPage, Tag};
use diesel::prelude::*;
use db;
use models;

pub struct DbExecutor(pub SqliteConnection);

impl Actor for DbExecutor {
   type Context = SyncContext<Self>;
}

// Message for requesting a particular conspiracy
pub struct GetConspiracy {
    pub page_id: String 
}

impl Message for GetConspiracy {
    type Result = Result<WikiPage, String>;
}

impl Handler<GetConspiracy> for DbExecutor {
   type Result = Result<WikiPage, String>;

   fn handle(&mut self, msg: GetConspiracy, _: &mut Self::Context) -> Self::Result
    {
        db::get_conspiracy_by_id(&self.0, &msg.page_id)
    }
}

// Message for returning a paged list of conspiracies
pub struct Conspiracies {
    pub page_num: i64
}

impl Message for Conspiracies {
    type Result = Result<Vec<WikiPage>, String>;
}

impl Handler<Conspiracies> for DbExecutor {
   type Result = Result<Vec<WikiPage>, String>;

   fn handle(&mut self, msg: Conspiracies, _: &mut Self::Context) -> Self::Result
    {
        db::get_conspiracies(&self.0, msg.page_num)
    }
}


pub struct AddTag {
    pub tag: models::NewTag
}

impl Message for AddTag {
    type Result = Result<usize, String>;
}

impl Handler<AddTag> for DbExecutor {
   type Result = Result<usize, String>;

   fn handle(&mut self, msg: AddTag, _: &mut Self::Context) -> Self::Result
    {
        match db::add_tag(&self.0, msg.tag) {
            Ok(res) => Ok(res),
            Err(e) => Err(format!("add_tag error {}", e))
        }
    }
}

pub struct Tags {
    pub page_num: i64
}

impl Message for Tags {
    type Result = Result<Vec<Tag>, String>;
}

impl Handler<Tags> for DbExecutor {
   type Result = Result<Vec<Tag>, String>;

   fn handle(&mut self, msg: Tags, _: &mut Self::Context) -> Self::Result
    {
        db::get_tags(&self.0, msg.page_num)
    }
}