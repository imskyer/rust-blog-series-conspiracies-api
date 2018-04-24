extern crate clap;     // brings the clap create into scope here
use clap::{App, Arg};  // brings the App and Arg structs from the clap create into this scope
use std::path::Path;   // Path is part of the standard lib so we only need the `use` statement to bring it into scope

fn main() {
    let _matches = App::new("conspiracies-db-loader")
       .version("0.0.1")
       .about("Parses and stores Wikipedia conspiracy theories data")
       .author("Rob Rowe.")
       .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .value_name("PAGE TITLE")
            .help("A title of a wikipage to retrieve")
            .takes_value(true)
            .required(true))
       .get_matches(); 

    if let Some(title) = _matches.value_of("title")  {
        println!("The title was passed in: {} (Hopefully, this is a Wikipage title).", title);
        
        // calling Wikipedia using the wikipedia crate will happen here
    } 
}