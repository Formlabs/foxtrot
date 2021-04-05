use rand::{Rng, SeedableRng};
use cdt::triangulate::Triangulation;

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
    while t.step() {}
    println!("{}", t.to_svg());
}

fn fuzz(n: usize) {
    loop {
        let seed: u64 = rand::thread_rng().gen();
        println!("Got seed {}", seed);
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

    benchmark(seed, 1_000_000);
    //fuzz(100);
    //svg(seed, 100);
}
