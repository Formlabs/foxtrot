use clap::{Arg, App};

use step::triangulate;
use step::parse::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("step_to_stl")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Converts a STEP file to a stl file")
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("stl file to target")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let start = std::time::SystemTime::now();
    let stripped_str = striped_string_from_path(input);
    let parsed = parse_entities_from_striped_file(&stripped_str);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Loaded + parsed in {:?}", since_the_epoch);

    let start = std::time::SystemTime::now();
    let tri = triangulate::Triangulator::run(&parsed);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Triangulated in {:?}", since_the_epoch);

    if let Some(o) = matches.value_of("output") {
        tri.save_stl(o)?;
    }

    Ok(())
}
