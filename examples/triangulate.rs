use std::iter::repeat_with;
use rand::{Rng, SeedableRng};

use clap::{Arg, App};
use itertools::Itertools;

const N: usize = 1_000_000;

fn main() {
    let matches = App::new("triangulate")
        .author("Matt Keeter <matt.j.keeter@gmail.com>")
        .about("Triangulates random points")
        .arg(Arg::with_name("count")
            .short("c")
            .long("count")
            .help("number of points")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("svg file to target")
            .takes_value(true))
        .arg(Arg::with_name("seed")
            .short("s")
            .long("seed")
            .help("seed for RNG")
            .takes_value(true))
        .get_matches();

    let count = matches.value_of("count")
        .map(|s| s.parse().expect("Could not parse count"))
        .unwrap_or(N);
    let seed: u64 = matches.value_of("seed")
        .map(|s| s.parse().expect("Could not parse count"))
        .unwrap_or_else(|| rand::thread_rng().gen());

    // Use a ChaCha RNG to be reproducible across platforms
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let points: Vec<(f64, f64)> = repeat_with(|| rng.gen_range(0.0..1.0))
        .tuple_windows()
        .take(count)
        .collect();

    let now = std::time::Instant::now();
    let result = cdt::triangulate(&points)
        .expect("No triangulation exists for this input.");
    let elapsed = now.elapsed();

    println!(
        "Triangulated {} points in {}.{}s.\nGenerated {} triangles.",
        count,
        elapsed.as_secs(),
        elapsed.subsec_millis(),
        result.len(),
    );
}
