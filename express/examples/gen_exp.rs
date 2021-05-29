use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

use clap::{Arg, App};
use express::parse::{strip_comments_and_lower, parse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("parse_exp")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Parses an EXPRESS file")
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("disable output"))
        .arg(Arg::with_name("output")
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file");

    let mut f = File::open(input).expect("file opens");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("read ok");

    let start = SystemTime::now();
    let s = strip_comments_and_lower(&buffer);
    let mut parsed = match parse(&s) {
        Ok(o) => o,
        Err(e) => panic!("Failed to parse:\n{:?}", e),
    };
    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    eprintln!("parsed in {:?}", since_the_epoch);

    let start = SystemTime::now();
    let gen = express::gen::gen(&mut parsed.1)?;
    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    eprintln!("generated in {:?}", since_the_epoch);

    match matches.value_of("output") {
        Some(o) => std::fs::write(o, gen)?,
        None => if !matches.is_present("quiet") {
            println!("{}", gen)
        },
    }
    Ok(())
}

