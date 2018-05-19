extern crate conspiracies;
extern crate clap; 
extern crate diesel;
extern crate dotenv;
extern crate actix;
extern crate actix_web;
extern crate futures;
#[macro_use] extern crate serde_derive;

use actix::{Addr,Syn};
use actix::prelude::*;
use conspiracies::actors::{AddTag, Conspiracies, DbExecutor,GetConspiracy, Tags};
use actix_web::{http, Json, middleware, App, AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Path};
use actix_web::server::HttpServer;
use futures::Future;
use actix_web::Error;
use diesel::prelude::*;
/// This is state where we will store *DbExecutor* address.
struct State {
    db: Addr<Syn, DbExecutor>,
}

fn get_tags(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let page_num = req.query().get("page").unwrap_or("0").parse::<i64>().unwrap();
    
    req.state().db.send(Tags{page_num: page_num})
      .from_err()
      .and_then(|res| {
          match res {
              Ok(tags) => Ok(HttpResponse::Ok().json(tags)),
              Err(_) => Ok(HttpResponse::InternalServerError().into())
          }
      })
      .responder()
}

fn get_conspiracies(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let page_num = req.query().get("page").unwrap_or("0").parse::<i64>().unwrap();
    
    req.state().db.send(Conspiracies{page_num: page_num})
      .from_err()
      .and_then(|res| {
          match res {
              Ok(conspiracies) => Ok(HttpResponse::Ok().json(conspiracies)),
              Err(_) => Ok(HttpResponse::InternalServerError().into())
          }
      })
      .responder()
}

fn get_conspiracies_by_id(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let page_id = &req.match_info()["page_id"];
    // Send message to `DbExecutor` actor
    req.state().db.send(GetConspiracy{page_id: page_id.to_owned()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(conspiracy) => Ok(HttpResponse::Ok().json(conspiracy)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}


fn main() {
    let sys = actix::System::new("conspiracies-api");

    // Start 3 parallel db executors
    let addr = SyncArbiter::start(3, || {
        DbExecutor(SqliteConnection::establish("database/conspiracies.sqlite3").unwrap())
    });

    // Start http server
    HttpServer::new(move || {
        App::with_state(State{db: addr.clone()})
            .resource("/tags", |r| r.method(http::Method::GET).a(get_tags))
            .resource("/conspiracies", |r| r.method(http::Method::GET).a(get_conspiracies))
            .resource("/conspiracies/{page_id}", |r| r.method(http::Method::GET).a(get_conspiracies_by_id))})
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
