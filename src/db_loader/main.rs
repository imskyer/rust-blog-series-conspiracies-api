
// extern crate calls bring the crate into scope for this file

extern crate conspiracies;
extern crate clap; 
extern crate wikipedia;
extern crate dotenv;
// brings the App trait from the clap create into this scope
// The use statements bring structs, enums, functions, etc 
// so that you don't have to use their fully qualified names
// As an example the line below allows me to use App::<fn name>
// instead of clap::App::<fn name>
//use clap::{App, Arg};
use std::process;

use wikipedia::{Wikipedia};
use conspiracies::wiki::{WikiPage, WikiRepo, get_page_links};
use conspiracies::db;
use dotenv::dotenv;
use std::env;

fn main() {
    let _matches = clap::App::new("conspiracies")
       .version("0.0.1")
       .about("Parses and stores Wikipedia conspiracy theories data")
       .author("Rob Rowe.")
       .arg(clap::Arg::with_name("title")
            .short("t")
            .long("title")
            .value_name("PAGE TITLE")
            .help("A title of a wikipage to retrieve")
            .takes_value(true)
            .required(true))
       .get_matches(); 

    if let Some(title) = _matches.value_of("title")  {
        if title == "" {
            println!("Cannot search for an emtpy title!");
            process::exit(1);
        }
        println!("The title was passed in: {} (Hopefully, this is a Wikipage title).", title);
        
        // This gets the wiki client, which is an HTTP client. 
        let c = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        let wiki_repo = WikiRepo::new(&c) ;

        match wiki_repo.get_page(title.to_string()) {
            Err(e) => println!("ERROR: {}", e),
            Ok(p) => {
                // reads the .env file and adds any variables found there
                // to the env vars in the 'real' env.
                dotenv().ok();
                let database_url = env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set");
                let conn = db::get_sqlite_connection(database_url);
                match db::add_conspiracy(conn, p) {
                    Err(e) => println!("INSERT ERROR: {}", e),
                    Ok(_) => println!("Inserted the List Page, now getting links and fetchig pages")
                }
            }
        };


        let p2 = c.page_from_title(title.to_string());
        let links_iter = p2.get_links().expect("unable to get the links");
        for (i, l) in links_iter.enumerate() {
            println!("i:{} l: {}", i, l.title);
        }
    } 
}

