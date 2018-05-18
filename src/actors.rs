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
        // let a = WikiPage::new("a".to_owned(), "1".to_owned(), "sum".to_owned(), "cont".to_owned(), "back".to_owned());
        // let b = WikiPage::new("b".to_owned(), "2".to_owned(), "sum".to_owned(), "cont".to_owned(), "back".to_owned());
        // Ok(vec![a, b])
    }
}