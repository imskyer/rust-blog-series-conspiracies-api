extern crate conspiracies;
extern crate clap; 
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate futures;
#[macro_use] 
extern crate serde_derive;

use actix::{Addr,Syn};
use actix::prelude::*;
use conspiracies::actors::{
    tags::{AddTag, Tags}, 
    conspiracies::*, 
    db_executor::*,
};
use actix_web::{http, App, AsyncResponder, HttpRequest, HttpResponse};
use actix_web::server::HttpServer;
use futures::Future;
use actix_web::Error;
use actix_web::Json;
use actix_web::middleware::Logger;
use diesel::prelude::*;
use conspiracies::models;

/// This is state where we will store *DbExecutor* address.
struct State {
    db: Addr<Syn, DbExecutor>,
}


//(query, json): (Query<..>, Json<MyStruct)
fn add_tag((req, tag): (HttpRequest<State>, Json<models::NewTag>)) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.state().db.send(AddTag{tag: models::NewTag::new_tag(tag.name.to_owned())})
      .from_err()
      .and_then(|res| {
          match res {
              Ok(i) => Ok(HttpResponse::Ok().json(i)),
              Err(e) => {
                  println!("add_tag error: {}", e);
                  Ok(HttpResponse::InternalServerError().into())
              }
          }
      })
      .responder()
}
/// Returns a paginated list of tags that are available
fn get_tags(req: HttpRequest<State>) -> impl Future<Item=HttpResponse, Error=Error> {
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

/// Returns a paginated list of conspriacies. IF no page size is given the default is 25
fn get_conspiracies(req: HttpRequest<State>) -> impl Future<Item=HttpResponse, Error=Error> {
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

/// returns the conspiracy by the given id
fn get_conspiracies_by_id(req: HttpRequest<State>) -> impl Future<Item=HttpResponse, Error=Error> {
    let page_id = &req.match_info()["page_id"];

    // Send message to `DbExecutor` actor
    req.state().db.send(GetConspiracy{page_id: page_id.to_owned()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(conspiracy) => Ok(HttpResponse::Ok().json(conspiracy)),
                Err(_) => Ok(HttpResponse::NotFound().into())
            }
        })
        .responder()
}

//(query, json): (Query<..>, Json<MyStruct)
fn tag_conspiracy((req, tag): (HttpRequest<State>, Json<models::ConspiracyTag>)) -> Box<Future<Item=HttpResponse, Error=Error>> {
    
    req.state().db.send(TagConspiracy{tag: tag.into_inner()})
      .from_err()
      .and_then(|res| {
          match res {
              Ok(i) => Ok(HttpResponse::Ok().json(i)),
              Err(e) => {
                  println!("tag_conspiracy error: {}", e);
                  Ok(HttpResponse::InternalServerError().into())
              }
          }
      })
      .responder()
}


fn index(_req: HttpRequest<State>) -> &'static str {
    "The men in black are REAL!"
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("conspiracies-api");
    // Start 3 parallel db executors
    let addr = SyncArbiter::start(3, || {
        DbExecutor(SqliteConnection::establish("database/conspiracies.sqlite3").unwrap())
    });
 
    // Start http server

    HttpServer::new(move || {
        App::with_state(State{db: addr.clone()})
            .middleware(Logger::default())
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .resource("/conspiracies/{page_id}", |r| r.method(http::Method::GET).a(get_conspiracies_by_id))
            .resource("/conspiracies/{page_id}/tag", |r| r.method(http::Method::POST).with(tag_conspiracy))
            .resource("/tags/new", |r| r.method(http::Method::POST).with(add_tag))
            .resource("/tags", |r| r.method(http::Method::GET).a(get_tags))
            .resource("/conspiracies", |r| r.method(http::Method::GET).a(get_conspiracies))})
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
