extern crate conspiracies;
extern crate clap; 
extern crate dotenv;
extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn hi(req: HttpRequest) -> &'static str {
      "The men in black are REAL!"
}

fn main() {
    server::new(
        || App::new()
            .resource("/", |r| r.f(hi)))
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .run();
}
