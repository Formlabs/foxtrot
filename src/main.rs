use cdt::triangulate::Triangulation;

fn main() {
    let pts = vec![
        (0.0, 0.0), (1.5, 0.0), (0.0, 1.0), (2.0, 2.0)
    ];
    let mut t = Triangulation::new(&pts);
    t.step();
    println!("{}", t.to_svg());
}
