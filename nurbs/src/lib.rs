#[allow(non_snake_case)]
pub struct KnotVector {
    U: Vec<f64>,
}

impl KnotVector {
    /// For basis functions of order `p + 1`, finds the span in the knot vector
    /// that is relevant for position `u`.
    ///
    /// ALGORITHM A2.1
    pub fn find_span(&self, p: usize, u: f64) -> usize {
        // U is [u_0, u_1, ... u_m]
        let m = self.U.len() - 1;
        let n = m - (p + 1); // max basis index

        if u == self.U[n + 1] {
            return n;
        }
        let mut low = p;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;
        while u < self.U[mid] || u >= self.U[mid + 1] {
            if u < self.U[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }

    /// Computes non-vanishing basis functions of order `p + 1` at point `u`.
    ///
    /// ALGORITHM A2.2
    pub fn basis_funs(&self, p: usize, u: f64) -> Vec<f64> {
        #[allow(non_snake_case)]
        let mut N = vec![0.0; p + 1];

        let mut left = vec![0.0; p + 1];
        let mut right = vec![0.0; p + 1];
        N[0] = 1.0;
        let i = self.find_span(p, u);
        for j in 1..=p {
            left[j] = u - self.U[i + 1 - j];
            right[j] = self.U[i + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                let temp = N[r] / (right[r + 1] + left[j - r]);
                N[r] = saved + right[r + 1] * temp;
                saved = left[j - r]*temp;
            }
            N[j] = saved;
        }
        N
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_span() {
        let k = KnotVector { U: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0] };
        assert!(k.find_span(0, 0.0) == 2);
        assert!(k.find_span(0, 0.99) == 2);
        assert!(k.find_span(1, 0.99) == 2);
        assert!(k.find_span(2, 0.99) == 2);
    }
}
