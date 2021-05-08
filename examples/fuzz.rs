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
        .arg(Arg::with_name("check")
            .short("c")
            .long("check")
            .help("check invariants after each step (slow)"))
        .arg(Arg::with_name("lock")
            .short("l")
            .long("lock")
            .help("lock three edges to test constrained triangulation"))
        .get_matches();

    let num = matches.value_of("num")
        .map(|s| s.parse())
        .unwrap_or(Ok(N))?;

    let check = matches.is_present("check");

    let mut i = 0;
    loop {
        if i % 1000 == 0 {
            eprintln!("{}", i);
        }
        i += 1;

        let seed = rand::thread_rng().gen();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        // We generate random points as f32, to make it more likely that
        // some will line up exactly on one axis or another, which can trigger
        // interesting edge cases.  Experimentally, we have X or Y collisions
        // at a rate of about one per 4K fuzzed samples.
        let points: Vec<_> = repeat_with(|| rng.gen_range(0.0..1.0))
            .tuple_windows()
            .map(|(a, b): (f32, f32)| (a as f64, b as f64))
            .take(num)
            .collect();

        // Generator to build the triangulation
        let gen = || if matches.is_present("lock") {
            cdt::Triangulation::new_with_edges(&points,
                &[(0, 1), (1, 2), (2, 0)])
        } else {
            cdt::Triangulation::new(&points)
        };

        let mut t = gen()?;
        t.check();
        let result = std::panic::catch_unwind(move || {
            while !t.done() {
                t.step().expect("Could not triangulate");
                if check {
                    t.check();
                }
            }
        });

        // Count how many steps we can do before failure
        if result.is_err() {
            let mut safe_steps = 0;
            for i in 0..points.len() {
                let mut t = gen()?;
                let result = std::panic::catch_unwind(move || {
                    for _ in 0..i {
                        t.step().expect("oh no");
                        if check {
                            t.check();
                        }
                    }
                });
                if result.is_ok() {
                    safe_steps = i;
                } else {
                    break;
                }
            }

            let mut t = gen()?;
            for _ in 0..safe_steps {
                t.step().expect("Failed too early");
            }

            if let Some(out) = matches.value_of("output") {
                eprintln!("    Saving {}", out);
                t.save_debug_svg(out).expect("Could not save SVG");
            } else {
                println!("{}", t.to_svg(true));
            }
            eprintln!("Crashed with seed: {}", seed);
            break Ok(())
        }
    }
}
