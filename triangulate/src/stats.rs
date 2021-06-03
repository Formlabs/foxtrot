#[derive(Default)]
pub struct Stats {
    pub num_shells: usize,
    pub num_faces: usize,
    pub num_errors: usize,
    pub num_panics: usize,
}

impl Stats {
    // Combine two triangulations with an associative binary operator
    // (why yes, this _is_ a monoid)
    pub fn combine(mut a: Self, b: Self) -> Self {
        a.num_shells += b.num_shells;
        a.num_faces += b.num_faces;
        a.num_errors += b.num_errors;
        a.num_panics += b.num_panics;
        a
    }
}
