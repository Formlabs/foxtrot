use std::iter::repeat_with;
use rand::{Rng, SeedableRng};

use clap::{Arg, App};
use itertools::Itertools;

const N: usize = 64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("fuzz")
        .author("Matt Keeter <matt.j.keeter@gmail.com>")
        .about("Fuzzes the triangulator")
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
        .get_matches();

    let num = matches.value_of("num")
        .map(|s| s.parse())
        .unwrap_or(Ok(N))?;

    let mut i = 0;
    loop {
        if i % 1000 == 0 {
            eprintln!("{}", i);
        }
        i += 1;

        let seed = rand::thread_rng().gen();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let points: Vec<(f64, f64)> = repeat_with(|| rng.gen_range(0.0..1.0))
            .tuple_windows()
            .take(num)
            .collect();

        // Generator to build the triangulation
        let gen = || cdt::triangulate::Triangulation::new(&points)
            .expect("Failed to make triangulation");

        let mut t = gen();
        let result = std::panic::catch_unwind(move || {
            t.run().expect("Could not triangulate")
        });

        // Count how many steps we can do before failure
        if result.is_err() {
            let mut safe_steps = 0;
            for i in 0..points.len() {
                let mut t = gen();
                let result = std::panic::catch_unwind(move || {
                    for _ in 0..i {
                        t.step().expect("oh no");
                    }
                });
                if result.is_ok() {
                    safe_steps = i;
                } else {
                    break;
                }
            }

            let mut t = gen();
            for _ in 0..safe_steps {
                t.step().expect("Failed too early");
            }

            if let Some(out) = matches.value_of("output") {
                eprintln!("    Saving {}", out);
                t.save_svg(out).expect("Could not save SVG");
            } else {
                println!("{}", t.to_svg());
            }
            eprintln!("Crashed with seed: {}", seed);
            break Ok(())
        }
    }
}
