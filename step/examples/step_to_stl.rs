use clap::{Arg, App};

use step::ap214;
use step::triangulate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("step_to_stl")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Converts a STEP file to a stl file")
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("stl file to target")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let data = std::fs::read(input)?;
    let parsed = ap214::parse(&data);
    let tri = triangulate::Triangulator::run(&parsed);
    tri.save_stl(matches.value_of("output").expect("Need output file"))?;

    Ok(())
}
