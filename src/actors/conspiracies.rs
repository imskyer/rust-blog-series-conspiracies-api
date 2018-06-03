use actix::prelude::*;
use db;
use diesel::prelude::*;
use models::{Conspiracy};
use super::db_executor::{DbExecutor};
// Message for returning a paged list of conspiracies
pub struct Conspiracies {
    pub page_num: i64
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
