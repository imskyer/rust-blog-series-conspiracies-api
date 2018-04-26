// extern crate calls bring the crate into scope for this file
extern crate clap; 
extern crate wikipedia;

// brings the App trait from the clap create into this scope
// The use statements bring structs, enums, functions, etc 
// so that you don't have to use their fully qualified names
// As an example the line below allows me to use App::<fn name>
// instead of clap::App::<fn name>
//use clap::{App, Arg};
use std::process;


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
        let _wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
        // Retrieves the page data 
        let _page = _wiki.page_from_title(title.to_string());
        match _page.get_pageid() {
            Err(e) => println!("ERROR There was a problem getting the page_id for {}: {}", title.to_string(), e),
            Ok(page_id) => {
                if page_id == "-1" {
                    println!("404 Could not find a page with the title {}", title.to_string());
                    process::exit(1);
                } 
                
                println!("page: {:#?}", _page.get_links().unwrap().count());
            }
        }
    

    } 
}
