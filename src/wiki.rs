extern crate chrono;
//use diesel;
use diesel::{Insertable};
use diesel::deserialize::{FromSql};
use super::schema::{conspiracies, links_processed, categories_to_pages};
use wikipedia;
use wikipedia::iter::Iter;
use wikipedia::{Wikipedia, Page};
use std::{process, thread, time};
use rand::{Rng, thread_rng};
use wiki::chrono::prelude::*;
use self::chrono::{Local, DateTime, TimeZone};


// I've added the derive debug so I can use println! to print it out
#[derive(Insertable, Debug, Clone)]
#[table_name="conspiracies"]
pub struct WikiPage  {
    pub title: String, 
    pub page_id: String,
    summary: String,
    content: String,
    background: String, 
}

impl WikiPage {
    pub fn new(title: String, page_id: String, summary: String, content: String, background: String) -> WikiPage {
        WikiPage{
            title: title,
            page_id: page_id,
            summary: summary,
            content: content,
            background: background,
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name="links_processed"]
pub struct LinkProcessed  {
    pub title: String, 
    pub processed: i32,
}

#[derive(Insertable, Debug)]
#[table_name="categories_to_pages"]
pub struct CategoryToPage {
    page_id: String,
    pub category: String,      
}

impl CategoryToPage {
    pub fn new(page_id: &str, category: String) -> CategoryToPage {
        CategoryToPage {
            page_id: page_id.to_string(),
            category: category,
        }
    }
}

pub struct WikiRepo;

/// Handles interaction with the Wikipedia site
impl WikiRepo {
  
  /// get_page returns a WikiPage object that represents the page for the given title
  pub fn get_page<'a>(client: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>, title: String) -> Result<WikiPage, wikipedia::Error> {
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

            Ok(WikiPage::new(title.to_string(), page_id, summary, content, background))
        }
      }
  }
  // get_conspiracies takes a wikipedia conection, the title of the 'seed page' and an anonymous function that takes one parameter,
  // a WikiPage object.  The function is responsible for making the call to the db to save the conspiracy to the database.  The 
  // where F phrase says that any function that takes a WikiPage as a parameter is a valid type for F
  pub fn get_conspiracies<'a, F>(client: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>, links: Vec<LinkProcessed>, title: String, save_action: F) 
    where F: Fn(WikiPage, Vec<CategoryToPage>) {
    // This is here so I can grab the links from the listing page so I 
    // I can use the links to get the files
    let page = client.page_from_title(title.to_string()); 
    
    
    // Now I'm going to create a WikiPage object for the
    // Listing page so I can add it to the database
    match WikiRepo::get_page(client, title) {
        Err(e) => println!("SEED ERR: {}", e),
        // The save_action function is the closure I 
        Ok(seed_page) => {
            let mut categories: Vec<CategoryToPage> = Vec::new();
            let mut iter = page.get_categories().unwrap().map(|cat| {
                CategoryToPage {
                    page_id: page.get_pageid().unwrap().clone(),
                    category: cat.title,
                }
            });

            while let Some(c) = iter.next() {
                categories.push(c)
            }
            save_action(seed_page, categories);
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
                let page_id = p2.page_id.clone();
                let mut categories: Vec<CategoryToPage> = Vec::new();
                let mut iter = page.get_categories().unwrap().map(|cat| {
                    CategoryToPage {
                        page_id: page_id.clone(),
                        category: cat.title,
                    }
                });

                while let Some(c) = iter.next() {
                    categories.push(c)
                }
                save_action(p2, categories);
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

  pub fn get_and_store_links<'a, F>(client: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>, title: String, save_action: F)
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
