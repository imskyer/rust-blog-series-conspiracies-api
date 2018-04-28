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
use wikipedia::{Wikipedia};

struct WikiPage  {
    title: String, 
    page_id: String,
    summary: String,
    content: String,
    background: String, 
}

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
        let _wiki = WikiRepo {
            client: wikipedia::Wikipedia::<wikipedia::http::default::Client>::default(),
        };

        match _wiki.get_page(title.to_string()) {
            Ok(p) => println!("title: {}\npage_id: {}\nSummary: {}\n", p.title, p.page_id, p.summary),
            Err(e) => println!("ERROR: {}", e)
        };
    } 
}

struct WikiRepo {
    client: Wikipedia<wikipedia::http::default::Client>,
}

/// Handles interaction with the Wikipedia site
impl WikiRepo {
  /// get_page returns a WikiPage object that represents the page for the given title
  fn get_page(self, title: String) -> Result<WikiPage, wikipedia::Error> {
      let page = self.client.page_from_title(title.to_string());  
      
      match page.get_pageid() {
        Err(e) => Err(e),
        Ok(page_id) => {
            // background can be None sometimes but I want to store an empty 
            // string instead of some other value
            let background = match page.get_section_content("background").unwrap()  {
                Some(val) => val,
                None => String::from("")
            };
            
            Ok(WikiPage {
                title: title, 
                page_id: page_id,
                summary: page.get_summary().unwrap(),
                content: page.get_content().unwrap(),
                background:  background, 
            })
        }
      }
  }
}

