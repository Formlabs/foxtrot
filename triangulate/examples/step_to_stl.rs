use clap::{Arg, App};

use triangulate::triangulate::triangulate;
use step::step_file::StepFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = App::new("step_to_stl2")
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
    let data = std::fs::read(input)?;
    let flat = StepFile::strip_flatten(&data);
    let entities = StepFile::parse(&flat);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Loaded + parsed in {:?}", since_the_epoch);

    let start = std::time::SystemTime::now();
    let tri = triangulate(&entities);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Triangulated in {:?}", since_the_epoch);

    if let Some(o) = matches.value_of("output") {
        tri.0.save_stl(o)?;
    }

    Ok(())
}
