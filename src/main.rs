extern crate clap;     // brings the clap create into scope here
use clap::{App, Arg};  // brings the App and Arg structs from the clap create into this scope
use std::path::Path;   // Path is part of the standard lib so we only need the `use` statement to bring it into scope

fn main() {
    let _matches = App::new("conspiracies-db-loader")
       .version("0.0.1")
       .about("Parses and stores Wikipedia conspiracy theories data")
       .author("Rob Rowe.")
       .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("PATH")
            .help("A path to a JSON doc ready to be parsed")
            .takes_value(true)
            .required(true))
       .get_matches(); 

    if let Some(path) = _matches.value_of("input")  {
        println!("A path was passed in: {} (Pretend its a path, it hasn't been tested)", path);
        if !Path::new(path).exists() {
            println!("Path does not exst")
        }

        // Reading the file in and parsing its contents will happen here.
    } 
}