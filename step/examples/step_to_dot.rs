use clap::{Arg, App};
use step::step_file::StepFile;

pub fn to_dot(s: &StepFile) -> String {
    let mut out = "digraph {\n".to_owned();
    for (i, e) in s.0.iter().enumerate() {
        let d = format!("{:?}", e);
        let name = d.split("(").next().unwrap();

        out += &format!("  e{} [ label = \"#{}: {}\" ];\n", i, i, name);
        for j in e.upstream() {
            out += &format!("  e{} -> e{};\n", i, j);
        }
    }
    out += "}";
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("step_to_dot2")
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

    let start = std::time::SystemTime::now();
    let data = std::fs::read(input)?;
    let flat = StepFile::strip_flatten(&data);
    let entities = StepFile::parse(&flat);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Loaded + parsed in {:?}", since_the_epoch);

    let dot = to_dot(&entities);
    if let Some(out) = matches.value_of("output") {
        std::fs::write(out, dot)?;
    } else {
        println!("{}", dot);
    }
    Ok(())
}

