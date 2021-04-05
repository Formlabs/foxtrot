use rand::{Rng, SeedableRng};
use cdt::triangulate::Triangulation;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: u64 = if args.len() == 2 {
        args[1].parse().expect("Could not parse seed")
    } else {
        let mut rng = rand::thread_rng();
        rng.gen()
    };
    eprintln!("Seed: {}", seed);

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut pts = Vec::new();
    for _ in 0..70 {
        pts.push((rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)));
    }
    let mut t = Triangulation::new(&pts);
    while t.step() {}
    println!("{}", t.to_svg());
}
