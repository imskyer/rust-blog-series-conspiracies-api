pub mod actors;
pub mod db;
pub mod models;
pub mod schema;
pub mod wiki;

#[macro_use] extern crate diesel;
extern crate wikipedia;
extern crate rand;
extern crate chrono;
extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;