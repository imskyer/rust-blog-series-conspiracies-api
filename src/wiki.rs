//use diesel;
use diesel::{Insertable};
use super::schema::conspiracies;
use wikipedia;
use wikipedia::iter::Iter;
use std;
use wikipedia::{Wikipedia, Page};


// I've added the derive debug so I can use println! to print it out
#[derive(Insertable, Debug, Clone)]
#[table_name="conspiracies"]
pub struct WikiPage  {
    title: String, 
    page_id: String,
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

pub struct WikiRepo<'a> {
    //pub client: Wikipedia<wikipedia::http::default::Client>,
    c: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>,
}

/// Handles interaction with the Wikipedia site
impl <'a> WikiRepo<'a> {
  pub fn new(c: &'a wikipedia::Wikipedia::<wikipedia::http::default::Client>) -> WikiRepo {
      WikiRepo {
          //client: Wikipedia<wikipedia::http::default::Client>::default(),
          c: c,
      }
  }
  /// get_page returns a WikiPage object that represents the page for the given title
  pub fn get_page(self, title: String) -> Result<WikiPage, wikipedia::Error> {
      let page = self.c.page_from_title(title.to_string());  
      
      match page.get_pageid() {
        Err(e) => Err(e),
        Ok(page_id) => {
            // background can be None sometimes but I want to store an empty 
            // string instead of some other value
            let background = match page.get_section_content("background").unwrap()  {
                Some(val) => val,
                None => String::from("")
            };
            
            Ok(WikiPage::new(title, page_id, page.get_summary().unwrap(), page.get_content().unwrap(), background))
        }
      }
  }  
}

pub fn get_page_links<F>(client:  Wikipedia<wikipedia::http::default::Client>, title: String, action: F)  
    where F: Fn(Page<wikipedia::http::default::Client>) {
  let page = client.page_from_title(title.to_string()); 
  action(page);
}
