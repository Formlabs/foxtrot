use clap::{Arg, App};

use step::parse::flatten;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("step_to_dot")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Tests STEP flattening")
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("dot file to target")
            .takes_value(true))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("disable output"))
        .arg(Arg::with_name("input")
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let data = std::fs::read(input)?;
    let parsed = flatten(&data);
    if !matches.is_present("quiet") {
        for p in parsed {
            println!("{}\n--------", std::str::from_utf8(p).unwrap());
        }
    }
    Ok(())
}

