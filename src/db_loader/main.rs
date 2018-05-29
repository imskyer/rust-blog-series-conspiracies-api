
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
use std::process;
use conspiracies::wiki::{WikiRepo};
use conspiracies::db;
use dotenv::dotenv;
use std::env;
//use wikipedia::http;

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
       .arg(clap::Arg::with_name("links")
            .short("gl")
            .long("get-links")
            .help("Gets links to conspiracy pages and stores them in the db")
            .takes_value(false)
            .required(false))
        .arg(clap::Arg::with_name("page_count")
            .short("pc")
            .long("page-count")
            .help("limits the number of pages to retrieve to the given number.")
            .takes_value(true)
            .required(false))
       .get_matches(); 

    if let Some(title) = _matches.value_of("title")  {
        if title == "" {
            println!("Cannot search for an emtpy title!");
            process::exit(1);
        }
        
        // get the batch size
        let mut batch_size = 60;
        if let Some(page_count) = _matches.value_of("page_count") {
            batch_size = page_count.trim().parse::<i32>().unwrap();
            println!("fetching at most {} pages.", batch_size);
        }

        // reads the .env file and adds any variables found there
        // to the env vars in the 'real' env.
        dotenv().ok();

        // This gets the wiki client, which is an HTTP client. 
        let c = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();

        // Database connection 
        let database_url = env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set");
        let conn = db::get_sqlite_connection(database_url);
        
        if _matches.is_present("get-links") {
            WikiRepo::get_page_links(&c, title.to_string(), |link| {
                match db::add_link_process(&conn, &link) {
                    Err(e) => println!("SAVE ERROR: {} {}", e ,link.title),
                    Ok(_) => println!("Added: {}", link.title)
                };
            });
        }

        let links =  db::get_links_to_process(&conn, batch_size);
        WikiRepo::get_conspiracies(&c, links, title.to_string(), |p2, categories| {
            match db::add_conspiracy(&conn, &p2) {
                Err(e) => println!("SAVE ERROR: {} {} {}", e ,p2.title, p2.page_id),
                Ok(_) => {
                    let title = &p2.title;
                    db::add_categories(&conn, categories).unwrap();
                    db::mark_link_as_processed(&conn, title).expect(&format!("A problem occurred when marking the link '{}' as processed",title));
                    println!("Added: {} {}", title, p2.page_id)
                }
            };
        });
    } 
}
