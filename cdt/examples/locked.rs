use std::iter::repeat_with;
use rand::{Rng, SeedableRng};

use clap::{Arg, App};
use itertools::Itertools;

const N: usize = 128;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("locked")
        .author("Matt Keeter <matt.j.keeter@gmail.com>")
        .about("Triangulates random points with a locked triangle")
        .arg(Arg::with_name("num")
            .short("n")
            .long("num")
            .help("number of points")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("svg file to target")
            .takes_value(true))
        .arg(Arg::with_name("check")
            .short("c")
            .long("check")
            .help("check invariants after each step (slow)"))
        .arg(Arg::with_name("seed")
            .short("s")
            .long("seed")
            .help("seed for RNG")
            .takes_value(true))
        .get_matches();

    let num = matches.value_of("num")
        .map(|s| s.parse())
        .unwrap_or(Ok(N))?;
    let seed: u64 = matches.value_of("seed")
        .map(|s| s.parse())
        .unwrap_or_else(|| Ok(rand::thread_rng().gen()))?;

    // Use a ChaCha RNG to be reproducible across platforms
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    // Sample as f32 to match the behavior in fuzz.rs
    // (to increase likelihood of collisions)
    let points: Vec<_> = repeat_with(|| rng.gen_range(0.0..1.0))
        .tuple_windows()
        .map(|(a, b): (f32, f32)| (a as f64, b as f64))
        .take(num)
        .collect();

    eprintln!("Running with seed {}", seed);
    let now = std::time::Instant::now();
    let mut t = cdt::Triangulation::new_with_edges(&points,
        &[(0, 1), (1, 2), (2, 0)])?;
    while !t.done() {
        t.step()?;
        if matches.is_present("check") {
            t.check();
        }
    }
    let result = t.triangles().collect::<Vec<_>>();
    let elapsed = now.elapsed();

    eprintln!(
        "Triangulated {} points in {}.{}s.\nGenerated {} triangles.",
        num,
        elapsed.as_secs(),
        elapsed.subsec_millis(),
        result.len(),
    );

    if let Some(out) = matches.value_of("output") {
        eprintln!("    Saving {}", out);
        t.save_debug_svg(out).expect("Could not save SVG");
    }

    Ok(())
}

