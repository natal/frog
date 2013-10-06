use std::num::Zero;
use nalgebra::na::DMat;

#[inline(always)]
pub fn dtw<V1,
           V2,
           N: Zero + Bounded + Add<N, N> + Clone + Orderable>(
           a: &[V1],
           b: &[V2],
           d: &fn(&V1, &V2) -> N)
           -> N {
    let nrows = a.len() + 1;
    let ncols = b.len() + 1;
    let mut dtw: DMat<N> = DMat::from_elem(nrows, ncols, Bounded::max_value());

    dtw.set(0, 0, Zero::zero());

    for i in range(1u, a.len() + 1) {
        for j in range(1u, b.len() + 1) {
            let min = {
                let down      = &dtw.at(i - 1, j);
                let left      = &dtw.at(i, j - 1);
                let down_left = &dtw.at(i - 1, j - 1);

                down.min(&left.min(down_left))
            };

            dtw.set(i, j, d(&a[i - 1], &b[j - 1]) + min);
        }
    }

    dtw.at(a.len() - 1, b.len() - 1)
}
