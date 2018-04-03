extern crate clap; // brings the clap create into scope here
use clap::{App, Arg};     // brings the App trait from the clap create into this scope

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

    if let Some(path) = _matches.value_of("input") {
        println!("A path was passed in: {} (Pretend its a path, it hasn't been tested)", path);
    }
}
