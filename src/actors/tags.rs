use actix::prelude::*;
use db;
use models::{NewTag, Tag};
use super::db_executor::{DbExecutor};

pub struct AddTag {
    pub tag: NewTag
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
    pub page_num: i32
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