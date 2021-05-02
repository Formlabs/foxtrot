use clap::{Arg, App};
use rusttype::{point, Font, Scale, OutlineBuilder};

#[derive(Default)]
struct Builder {
    points: Vec<(f64, f64)>,
    contours: Vec<Vec<usize>>,
    x: f32, y: f32,
    dx: f32, dy: f32,
}

impl Builder {
    fn set_offset(&mut self, dx: f32, dy: f32) {
        self.dx = dx;
        self.dy = dy;
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = -y;
        self.points.push(((x + self.dx) as f64, (self.dy - y) as f64));
    }
}

impl OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        // Begin a new contour
        self.contours.push(vec![self.points.len()]);
        self.set_position(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        // Push a move to this point into the last contour
        self.contours.last_mut().unwrap().push(self.points.len());
        self.set_position(x, y);
    }

    fn close(&mut self) {
        // Remove last coordinate + point (which is a duplicate), then reassign
        let c = self.contours.last_mut().unwrap();
        *c.last_mut().unwrap() = c[0];
        self.points.pop().unwrap();

        // Leave position unchanged since we're going to start a new contour
        // shortly (if all is behaving well)
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // TODO
        //self.line_to(x1, y1);
        self.line_to(x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // TODO
        //self.line_to(x1, y1);
        //self.line_to(x2, y2);
        self.line_to(x, y);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("font")
        .author("Matt Keeter <matt.j.keeter@gmail.com>")
        .about("Triangulates a few characters from a font")
        .arg(Arg::with_name("font")
            .short("f")
            .long("font")
            .help("path to the font TTF")
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
        .arg(Arg::with_name("text")
            .short("t")
            .long("text")
            .help("text to triangulate")
            .takes_value(true))
        .get_matches();

    let font_path = matches.value_of("font")
        .unwrap_or("/Library/Fonts/Arial.ttf");
    let font = {
        let data = std::fs::read(&font_path)?;
        Font::try_from_vec(data).unwrap()
    };

    let text = matches.value_of("text").unwrap_or("hello, world");

    // Load the font glyphs into the triangulation builder
    let mut builder = Builder::default();
    let scale = Scale::uniform(10.0);
    for g in font.layout(text, scale, point(0.0, 0.0)) {
        let pos = g.position();
        builder.set_offset(pos.x, pos.y);
        g.unpositioned().build_outline(&mut builder);
    }

    // Then, do the work of triangulation
    let now = std::time::Instant::now();
    let mut t = cdt::Triangulation::new_from_contours(
        &builder.points,
        &builder.contours)?;
    while !t.done() {
        t.step()?;
        if matches.is_present("check") {
            t.check();
        }
    }
    let result = t.triangles().collect::<Vec<_>>();
    let elapsed = now.elapsed();

    eprintln!(
        "    Triangulated '{}' in {}.{}s.\n    Generated {} triangles.",
        text,
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
