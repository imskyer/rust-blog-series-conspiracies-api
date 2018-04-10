// brings the clap crate into scope here
extern crate clap; 
// brings the serde_json crate into scope
extern crate serde_json;

// brings the App trait from the clap create into this scope
use clap::{App, Arg};
// Path is part of the standard lib so we only need the 
// `use` statement to bring it into scope 
use std::path::Path;
// for the process::exit call in the main fn
use serde_json::{Value};
use std::process;
use std::fs::File;
use std::io::Read;

fn main() {
    let _matches = App::new("conspiracies-db-loader")
       .version("0.0.1")
       .about("Parses and stores Wikipedia conspiracy theories data")
       .author("Rob Rowe")
       .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("PATH")
            .help("A path to a JSON doc ready to be parsed")
            .takes_value(true)
            .required(true))
       .get_matches(); 

    if let Some(input_value) = _matches.value_of("input")  {
        
        let path = Path::new(input_value);
        if !path.exists() {
            println!("--input value is not a valid path");
            process::exit(1);
        }

        if path.is_dir() {
            println!("--input value is a path to a directory, not a file.");
            process::exit(1);
        }

        // reading the contents of the wikipedia query results I stored on my
        // file system, it is a JSON file.  I'm using a mutable var here since 
        // the read_to_string call will be updating the variable as it reads the
        // file's contents.
        let mut contents = String::new();
        let mut f = File::open(input_value).expect("file not found");
        f.read_to_string(&mut contents)
           .expect("something went wrong reading the file");

        let parsed_json: Value = serde_json::from_str(contents.as_str())
                                .expect("an error occurred while attempting to parse the JSON string");
        // when querying wikipedia the resulting JSON for the search the results are stored under a 
        // a root property called parse.  To make it easier to get to the actual text that I want to
        // work with I'm setting up a var that contains all info for the wikipedia page.
        let conspiracy_doc = &parsed_json["parse"];
        println!("title: {}\ncontent: {:?}\n", conspiracy_doc["title"], conspiracy_doc["text"]);
        println!("Successfully parsed the JSON!");
    } 
} 