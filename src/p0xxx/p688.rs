// Problem 688
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        #[inline]
        fn pos2idx(n: usize, r: usize, c: usize) -> usize {
            r * n + c
        }

        let n = n as usize;
        let row = row as usize;
        let col = column as usize;

        let mut prev = vec![0.0f64; n * n];
        let mut curr = prev.clone();

        prev[pos2idx(n, row, col)] = 1.0;

        for step in 0..k {
            if step >= 1 {
                curr.fill(0.0);
            }

            for i in 0..n {
                for j in 0..n {
                    let prob = prev[pos2idx(n, i, j)];

                    for (i2, j2) in [
                        (i.wrapping_sub(2), j.wrapping_sub(1)),
                        (i.wrapping_sub(2), j.wrapping_add(1)),
                        (i.wrapping_sub(1), j.wrapping_sub(2)),
                        (i.wrapping_sub(1), j.wrapping_add(2)),
                        (i.wrapping_add(1), j.wrapping_sub(2)),
                        (i.wrapping_add(1), j.wrapping_add(2)),
                        (i.wrapping_add(2), j.wrapping_sub(1)),
                        (i.wrapping_add(2), j.wrapping_add(1)),
                    ] {
                        if i2 >= n || j2 >= n {
                            continue;
                        }

                        curr[pos2idx(n, i2, j2)] += prob / 8.0;
                    }
                }
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        prev.iter().sum()
    }
}
