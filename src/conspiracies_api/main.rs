extern crate conspiracies;
extern crate clap; 
extern crate diesel;
extern crate dotenv;
extern crate actix;
extern crate actix_web;
extern crate futures;

use actix::{Addr,Syn};
use actix::prelude::*;
use conspiracies::actors::{Conspiracies, DbExecutor,GetConspiracy};
use actix_web::{http, middleware, App, AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Path};
use actix_web::server::HttpServer;
use futures::Future;
use actix_web::Error;
use diesel::prelude::*;
/// This is state where we will store *DbExecutor* address.
struct State {
    db: Addr<Syn, DbExecutor>,
}


fn hi(req: HttpRequest) -> &'static str {
      "The men in black are REAL!"
}

fn get_categories(req: HttpRequest) -> &'static str {
    "This will eventually return a list of the conspiracy categories"
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
            .resource("/conspiracies", |r| r.method(http::Method::GET).a(get_conspiracies))
            .resource("/conspiracies/{page_id}", |r| r.method(http::Method::GET).a(get_conspiracies_by_id))})
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
    // server::new(
    //     || App::new()
    //         .resource("/", |r| r.f(hi))
    //         .resource("/categories", |r| r.f(get_categories))
    //         .resource("/conspiracies", |r| r.f(get_conspiracies))
    //         .resource("/conspiracies/{page_id}", |r| r.method(http::Method::GET).with(get_conspiracies_by_id))
    //         )
    //     .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
    //     .run();
}
