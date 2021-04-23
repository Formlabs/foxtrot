use rand::{Rng, SeedableRng};
use cdt::triangulate::Triangulation;

#[allow(dead_code)]
fn benchmark(seed: Option<u64>, n: usize) {
    let seed = seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });
    eprintln!("Seed: {}", seed);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut pts = Vec::new();
    for _ in 0..n {
        pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
    }
    let mut t = Triangulation::new(&pts)
        .expect("Failed to make triangulation");
    t.run().expect("Failed to triangulate");
}

#[allow(dead_code)]
fn svg(seed: Option<u64>, n: usize) {
    let seed = seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });
    eprintln!("Seed: {}", seed);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut pts = Vec::new();
    for _ in 0..n {
        pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
    }
    let mut t = Triangulation::new(&pts).expect("Failed to make triangulation");
    t.run().expect("Failed to triangulate");
    println!("{}", t.to_svg());
}

#[allow(dead_code)]
const FUZZ_COUNT: usize = 32;
const FUZZ_EDGES: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 0)];

fn test_lock(seed: Option<u64>) {
    let seed = seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });
    eprintln!("Seed: {}", seed);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut pts = Vec::new();
    for _ in 0..FUZZ_COUNT {
        pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
    }
    let mut t = Triangulation::new_with_edges(&pts, &FUZZ_EDGES)
        .expect("Failed to make triangulation");
    t.run().expect("Failed to triangulate");
    println!("{}", t.to_svg());
}

#[allow(dead_code)]
fn fuzz_lock(seed: Option<u64>) {
    eprintln!("Running...");
    let mut i = 0;
    loop {
        let seed = seed.unwrap_or_else(|| {
            rand::thread_rng().gen()
        });

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        if i % 1000 == 0 {
            eprintln!("{}", i);
        }
        i += 1;

        let mut pts = Vec::new();
        for _ in 0..FUZZ_COUNT {
            pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
        }

        // Generator to build the triangulation
        let gen = || Triangulation::new_with_edges(&pts, &FUZZ_EDGES)
            .expect("Failed to make triangulation");
        let mut t = gen();
        let result = std::panic::catch_unwind(move || {
            t.run().expect("Could not triangulate")
        });
        if result.is_err() {
            let mut safe_steps = 0;
            for i in 0..pts.len() {
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
                t.step().expect("oh no");
            }
            println!("{}", t.to_svg());
            eprintln!("Crashed with seed: {}", seed);
            t.step().expect("uh oh"); // Triggers the crash again
            break;
        }
    }
}

#[allow(dead_code)]
fn fuzz(seed: Option<u64>, n: usize) {
    loop {
        let seed: u64 = seed.unwrap_or_else(|| rand::thread_rng().gen());
        eprintln!("Got seed {}", seed);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let mut pts = Vec::new();
        for _ in 0..n {
            pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
        }
        let mut t = Triangulation::new(&pts)
            .expect("Failed to make triangulation");
        t.run().expect("Failed to triangulate");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: Option<u64> = if args.len() == 2 {
        Some(args[1].parse().expect("Could not parse seed"))
    } else {
        None
    };

    //benchmark(seed, 1_000_000);
    //fuzz(seed, 5);
    //svg(seed, 64);
    test_lock(seed);
    //fuzz_lock(seed);
}
