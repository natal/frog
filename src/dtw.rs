use std::num::Zero;
use std::vec;

#[inline(always)]
pub fn dtw<V1,
           V2,
           N: Zero + Bounded + Add<N, N> + Clone + Orderable>(
           a: &[V1],
           b: &[V2],
           d: &fn(&V1, &V2) -> N)
           -> N {
    fn off(row: uint, col: uint, ncols: uint) -> uint{
        row * ncols + col
    }

    let nrows = a.len() + 1;
    let ncols = b.len() + 1;
    let mut dtw: ~[N] = vec::from_elem(nrows * ncols, Bounded::max_value());

    dtw[0] = Zero::zero();

    for i in range(1u, a.len() + 1) {
        for j in range(1u, b.len() + 1) {
            let min = {
                let down      = &dtw[off(i - 1, j, ncols)];
                let left      = &dtw[off(i, j - 1, ncols)];
                let down_left = &dtw[off(i - 1, j - 1, ncols)];

                down.min(&left.min(down_left))
            };

            dtw[off(i, j, ncols)] = d(&a[i - 1], &b[j - 1]) + min;
        }
    }

    dtw[off(a.len() - 1, b.len() - 1, ncols)]
}
