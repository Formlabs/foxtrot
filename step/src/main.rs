use std::fs::read_to_string;
// use std::io;
// use std::io::BufRead;
// use std::io::Read;
// use std::str;
// use std::collections::HashMap;
// use std::path::Path;

use step::parse_file::data_block;
use std::time::{SystemTime};

fn main() {

    let start = SystemTime::now();
    
    let filename = "/Users/Henry Heffan/Desktop/foxtrot/Kondo_only_data.step";
    let file = read_to_string(filename).expect("file opens");
    let (remainer, v) = data_block(&file[..(file.len() - 8)]).expect("file parsed");
    println!("{} {}", remainer.len(), v.len());

    let end = SystemTime::now();
    let since_the_epoch = end.duration_since(start).expect("Time went backwards");
    println!("time {:?}", since_the_epoch);
}

