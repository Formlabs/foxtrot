use rand::Rng;
use cdt::triangulate::Triangulation;

fn main() {
    let mut rng = rand::thread_rng();
    let mut pts = Vec::new();
    for _ in 0..20 {
        pts.push((rng.gen_range(0.0..20.0), rng.gen_range(0.0..20.0)));
    }
    let mut t = Triangulation::new(&pts);
    while t.step() {}
    println!("{}", t.to_svg());
}
