extern crate conspiracies;
extern crate clap; 
extern crate dotenv;
extern crate actix;
extern crate actix_web;

use actix_web::{http, server, App, HttpRequest, Path};

fn hi(req: HttpRequest) -> &'static str {
      "The men in black are REAL!"
}

fn get_categories(req: HttpRequest) -> &'static str {
    "This will eventually return a list of the conspiracy categories"
}

fn get_conspiracies(req: HttpRequest) -> &'static str {
    "This will eventually be a paginated list"
}

fn get_conspiracies_by_id(page_id: Path<(u32)>) -> String {
    format!("This will eventually be the contents of a conspiracy: {}", page_id.into_inner())
}

fn main() {
    server::new(
        || App::new()
            .resource("/", |r| r.f(hi))
            .resource("/categories", |r| r.f(get_categories))
            .resource("/conspiracies", |r| r.f(get_conspiracies))
            .resource("/conspiracies/{page_id}", |r| r.method(http::Method::GET).with(get_conspiracies_by_id))
            )
        .bind("127.0.0.1:8088").expect("Can not bind to 127.0.0.1:8088")
        .run();
}
