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