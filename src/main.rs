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
    let mut t = Triangulation::new(&pts);
    while t.step() {}
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
    let mut t = Triangulation::new(&pts);
    t.run();
    println!("{}", t.to_svg());
}

#[allow(dead_code)]
fn test_lock(seed: Option<u64>) {
    let seed = seed.unwrap_or_else(|| {
        rand::thread_rng().gen()
    });
    eprintln!("Seed: {}", seed);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut pts = Vec::new();
    for _ in 0..32 {
        pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
    }
    let mut t = Triangulation::new_with_edges(&pts, &[(0, 1), (1, 2), (2, 0)]);
    t.run();
    println!("{}", t.to_svg());
}

#[allow(dead_code)]
fn fuzz_lock(seed: Option<u64>) {
    eprintln!("Running...");
    loop {
        let seed = seed.unwrap_or_else(|| {
            rand::thread_rng().gen()
        });

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        eprintln!("Seed: {}", seed);

        let mut pts = Vec::new();
        for _ in 0..32 {
            pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
        }
        let mut t = Triangulation::new_with_edges(&pts, &[(0, 1), (1, 2), (2, 0)]);
        let result = std::panic::catch_unwind(move || {
            t.run();
        });
        if result.is_err() {
            let mut safe_steps = 0;
            for i in 0..pts.len() {
                let mut t = Triangulation::new_with_edges(&pts, &[(0, 1), (1, 2), (2, 0)]);
                let result = std::panic::catch_unwind(move || {
                    for _ in 0..i {
                        t.step();
                    }
                });
                if result.is_ok() {
                    safe_steps = i;
                } else {
                    break;
                }
            }

            let mut t = Triangulation::new_with_edges(&pts, &[(0, 1), (1, 2), (2, 0)]);
            eprintln!("\n\n");
            for _ in 0..safe_steps {
                t.step();
            }
            println!("{}", t.to_svg());
            eprintln!("Crashed with seed: {}", seed);
            t.step(); // Triggers the crash again
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
        let mut t = Triangulation::new(&pts);
        while t.step() {}
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
    //test_lock(seed);
    fuzz_lock(seed);
}
