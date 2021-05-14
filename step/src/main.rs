use std::time::SystemTime;
use step::parse::{striped_string_from_path, parse_entities_from_striped_file};

fn main() {
    let start = SystemTime::now();

    let filename = "/Users/Henry Heffan/Desktop/foxtrot/KondoMotherboard_RevB_full.step";
    let stripped_str = striped_string_from_path(filename);
    let entities = parse_entities_from_striped_file(&stripped_str);

    println!("{}", entities.0.len());

    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    println!("time {:?}", since_the_epoch);
}
