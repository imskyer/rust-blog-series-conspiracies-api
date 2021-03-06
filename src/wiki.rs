extern crate chrono;

use wikipedia;
use std::{thread, time};
use rand::{Rng, thread_rng};
use self::chrono::{Local, DateTime};
use models::{Conspiracy, LinkProcessed};

pub struct WikiRepo;

/// Handles interaction with the Wikipedia site
impl WikiRepo {
  
  /// get_page returns a Conspiracy object that represents the page for the given title
  pub fn get_page<'a>(client: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>, title: String) -> Result<Conspiracy, wikipedia::Error> {
      let page = client.page_from_title(title.to_string());  
      
      match page.get_pageid() {
        Err(e) => Err(e),
        Ok(page_id) => {
            // background can be None sometimes but I want to store an empty 
            // string instead of some other value
            let mut background = "".to_string();
            match page.get_section_content("background") {
                Ok(bg) => {background = bg.unwrap_or_default();},
                _ => ()
            };
            
            let summary = match page.get_summary() {
                Ok(val) => val,
                Err(e) => {
                    println!("ERROR get_summary {}", e);
                    String::from("")
                }
            };

            let content = match page.get_content() {
                Ok(val) => val,
                Err(e) => {
                    println!("ERROR get_content {}", e);
                    String::from("")
                }
            };

            Ok(Conspiracy::new(title.to_string(), page_id, summary, content, background))
        }
      }
  }
  // get_conspiracies takes a wikipedia conection, the title of the 'seed page' and an anonymous function that takes one parameter,
  // a Conspiracy object.  The function is responsible for making the call to the db to save the conspiracy to the database.  The 
  // where F phrase says that any function that takes a Conspiracy as a parameter is a valid type for F
  pub fn get_conspiracies<'a, F>(client: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>, links: Vec<LinkProcessed>, title: String, save_action: F) 
    where F: Fn(Conspiracy) {
    
    // Now I'm going to create a Conspiracy object for the
    // Listing page so I can add it to the database
    match WikiRepo::get_page(client, title) {
        Err(e) => println!("SEED ERR: {}", e),
        // The save_action function is the closure I 
        Ok(seed_page) => {
            save_action(seed_page); //, categories);
        }
    };

    // I'm looping over the Links that I retreived early
    // The Link struct has a single field, title, and that
    // is all I need.s to fetch the Wikipedia page.  
    let mut i =0;
    for link in links.into_iter() {
        //println!("get_conspiracies: {}", link.title);
        match WikiRepo::get_page(client, link.title) {
            Err(e) => println!("SAVING ERROR: {}", e),
            Ok(p2) => {
                save_action(p2);
                // I added the sleep code to slow down my requests
                // it appeared as if I was getting denied because of 
                // the frequency of my calls but after adding this I didn't
                // have that problem.
                if i % 10  == 0 {
                    let num = thread_rng().gen_range(60, 240);
                    let local: DateTime<Local> = Local::now(); 
                    println!("sleeping for {} seconds starting at {}", num, local);
                    thread::sleep(time::Duration::from_secs(num));
                }               
            }
        };
        i = i + 1;
    }
  } 

  pub fn get_page_links<'a, F>(client: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>, title: String, save_action: F)
        where F: Fn(LinkProcessed) {
        // This is here so I can grab the links from the listing page so I 
        // I can use the links to get the files
        let page = client.page_from_title(title.to_string()); 
        let links_iter = page.get_links().expect("unable to get the links");
        for (i, l) in links_iter.enumerate() {
            let lp = LinkProcessed {
                title: l.title,
                processed: 0,
            };
            save_action(lp);
        }
    } 
}
