use actix::prelude::*;
use db;
use diesel::prelude::*;
use models::{Conspiracy, ConspiracyTag};
use super::db_executor::{DbExecutor};
// Message for returning a paged list of conspiracies
pub struct Conspiracies {
    pub page_num: i32
}

impl Message for Conspiracies {
    type Result = Result<Vec<Conspiracy>, String>;
}

impl Handler<Conspiracies> for DbExecutor {
   type Result = Result<Vec<Conspiracy>, String>;

   fn handle(&mut self, msg: Conspiracies, _: &mut Self::Context) -> Self::Result
    {
        db::get_conspiracies(&self.0, msg.page_num)
    }
}

pub struct GetConspiraciesByTag {
    pub page_num : i32,
    pub tag_id : i32
}

impl Message for GetConspiraciesByTag {
    type Result = Result<Vec<Conspiracy>, String>;
}

impl Handler<GetConspiraciesByTag> for DbExecutor {
    type Result = Result<Vec<Conspiracy>, String>;

    fn handle(&mut self, msg: GetConspiraciesByTag, _: &mut Self::Context) -> Self::Result {
        println!("msg.page_num: {} msg.tag_id: {}", msg.page_num, msg.tag_id);
        db::get_conspiracies_by_tag_id(&self.0, msg.page_num, msg.tag_id)
    }
}

// Message for requesting a particular conspiracy
pub struct GetConspiracy {
    pub page_id: String 
}

impl Message for GetConspiracy {
    type Result = Result<Conspiracy, String>;
}

impl Handler<GetConspiracy> for DbExecutor {
   type Result = Result<Conspiracy, String>;

   fn handle(&mut self, msg: GetConspiracy, _: &mut Self::Context) -> Self::Result
    {
        db::get_conspiracy_by_id(&self.0, &msg.page_id)
    }
}

pub struct TagConspiracy {
    pub tag: ConspiracyTag 
}

impl Message for TagConspiracy {
    type Result = Result<usize, String>;
}

impl Handler<TagConspiracy> for DbExecutor {
   type Result = Result<usize, String>;

   fn handle(&mut self, msg: TagConspiracy, _: &mut Self::Context) -> Self::Result
    {
        match db::tag_conspiracy(&self.0, msg.tag) {
            Ok(u) => Ok(u),
            Err(e) => Err(format!("tag_conspiracy error: {}", e))
        }
    }
}