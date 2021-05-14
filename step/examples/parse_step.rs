use std::time::SystemTime;
use clap::{Arg, App};
use step::parse::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("parse_step")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Tests STEP parsing")
        .arg(Arg::with_name("input")
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let start = SystemTime::now();

    let stripped_str = striped_string_from_path(input);
    let entities = parse_entities_from_striped_file(&stripped_str);
    println!("Got {} entities", entities.0.len());

    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    println!("time {:?}", since_the_epoch);
    Ok(())
}
