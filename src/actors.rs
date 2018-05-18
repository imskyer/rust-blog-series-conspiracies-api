use actix::prelude::*;
use wiki::{WikiPage};
use diesel::prelude::*;
use db;

pub struct DbExecutor(pub SqliteConnection);

impl Actor for DbExecutor {
   type Context = SyncContext<Self>;
}

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


pub struct Categories {
    pub page_num: i64
}

impl Message for Categories {
    type Result = Result<Vec<String>, String>;
}

impl Handler<Categories> for DbExecutor {
   type Result = Result<Vec<String>, String>;

   fn handle(&mut self, msg: Categories, _: &mut Self::Context) -> Self::Result
    {
        db::get_categories(&self.0, msg.page_num)
    }
}