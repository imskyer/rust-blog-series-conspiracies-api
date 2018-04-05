// brings the clap create into scope here
extern crate clap; 
// brings the App trait from the clap create into this scope
use clap::{App, Arg};
// Path is part of the standard lib so we only need the 
// `use` statement to bring it into scope 
use std::path::Path;
// for the process::exit call in the main fn
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

        let contents = read_file(input_value);
        println!("With text:\n{}", contents);
    } 
} 

fn read_file(filepath: &str) -> String {
    //Here's an example of reading a file
    let mut f = File::open(filepath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}