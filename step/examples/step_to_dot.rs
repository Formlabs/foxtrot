use clap::{Arg, App};

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
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let data = std::fs::read(input)?;
    let parsed = step::parse::parse_file_as_string(&data);
    if let Some(out) = matches.value_of("output") {
        parsed.save_dot(out)?;
    } else {
        println!("{}", parsed.to_dot());
    }
    Ok(())
}
