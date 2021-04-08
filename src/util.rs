// Finds the four points in the given buffer with the lowest score, returning
// then in order (so that out[0] is closest)
//
// This is faster than sorting an entire array each time to find the four
// closest distances to a given point.
pub fn min4(buf: &[(usize, f64)]) -> [usize; 4] {
    let mut array = [(0, std::f64::INFINITY); 4];
    for &(p, score) in buf.iter() {
        if score >= array[3].1 {
            continue;
        }
        for i in 0..4 {
            // If the new score is bumping this item out of the array,
            // then shift all later items over by one and return.
            if score <= array[i].1 {
                for j in (i..3).rev() {
                    array[j + 1] = array[j];
                }
                array[i] = (p, score);
                break;
            }
        }
    }

    let mut out = [0usize; 4];
    for (i, a) in array.iter().enumerate() {
        out[i] = a.0;
    }
    out
}
