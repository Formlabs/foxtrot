use clap::{Arg, App};
use step::parse::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("step_to_dot")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Converts a STEP file to a dot file")
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("dot file to target")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let stripped_str = striped_string_from_path(input);
    let parsed = parse_entities_from_striped_file(&stripped_str);
    println!("Got {} entities", parsed.0.len());

    if let Some(out) = matches.value_of("output") {
        parsed.save_dot(out)?;
    } else {
        println!("{}", parsed.to_dot());
    }
    Ok(())
}
