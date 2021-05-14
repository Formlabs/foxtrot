use std::time::SystemTime;
use step::parse::parse_file_at_path;

fn main() {
    let start = SystemTime::now();

    let filename = "/Users/Henry Heffan/Desktop/foxtrot/KondoMotherboard_RevB_full.step";
    let entities = parse_file_at_path(filename);
    println!("{}", entities.0.len());

    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    println!("time {:?}", since_the_epoch);
}
