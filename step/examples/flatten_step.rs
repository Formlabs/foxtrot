use clap::{Arg, App};

use step::parse::{strip_flatten, into_blocks};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("flatten_step")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Tests STEP flattening")
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
    let stripped = strip_flatten(&data);
    let blocks = into_blocks(&stripped);
    if !matches.is_present("quiet") {
        for p in blocks {
            println!("{}\n--------", std::str::from_utf8(p).unwrap());
        }
    }
    Ok(())
}

