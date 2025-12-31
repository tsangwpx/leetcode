// Problem 3625
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        //https://leetcode.com/problems/count-number-of-trapezoids-ii/solutions/6980653/python-parallel-lines-and-midpoints-by-a-5gr1/
        /*
         * y=(dy/dx)x+c -> Ax+By+C= 0 where A = dx, B = -dy, C = c * dx
         * Given dx and dy are co-primes, "-C = dy * x - dx * y" can be treated as intercept somehow.
         *
         */
        use std::collections::HashMap;

        fn gcd(mut u: i32, mut v: i32) -> i32 {
            // https://en.wikipedia.org/wiki/Binary_GCD_algorithm
            if u == 0 {
                return v;
            } else if v == 0 {
                return u;
            }

            let i = u.trailing_zeros();
            let j = v.trailing_zeros();
            let k = i.min(j);

            u >>= i;
            v >>= j;

            loop {
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }

                v -= u;

                if v == 0 {
                    return u << k;
                }

                v >>= v.trailing_zeros();
            }
        }

        fn simplify(dy: i32, dx: i32) -> (i32, i32) {
            assert!(dy != 0 || dx != 0);

            if dy == 0 {
                (0, 1)
            } else if dx == 0 {
                (1, 0)
            } else {
                let sign = dy.signum() * dx.signum();
                let g = gcd(dy.abs(), dx.abs());

                (dy.abs() / g * sign, dx.abs() / g)
            }
        }

        #[inline]
        fn n_choose_two(n: i64) -> i64 {
            n * (n - 1) / 2
        }

        #[inline]
        fn sum_comb2_values<K>(map: &HashMap<K, i32>) -> i64 {
            map.values().copied().map(|s| n_choose_two(s as i64)).sum()
        }

        let mut slopes = HashMap::<(i32, i32), i32>::new();
        let mut colinears = HashMap::<(i32, i32, i32), i32>::new();
        let mut midpoints = HashMap::<(i32, i32), i32>::new();
        let mut midlines = HashMap::<(i32, i32, i32, i32, i32), i32>::new();

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let x1 = points[i][0];
                let y1 = points[i][1];
                let x2 = points[j][0];
                let y2 = points[j][1];

                let dx = x2 - x1;
                let dy = y2 - y1;
                let (dy, dx) = simplify(dy, dx);

                let intercept = dy * x1 - dx * y1;

                *slopes.entry((dy, dx)).or_default() += 1;
                *colinears.entry((dy, dx, intercept)).or_default() += 1;
                *midpoints.entry((x1 + x2, y1 + y2)).or_default() += 1;
                *midlines
                    .entry((x1 + x2, y1 + y2, dx, dy, intercept))
                    .or_default() += 1;
            }
        }

        let mut res = 0i64;
        res += sum_comb2_values(&slopes);
        res -= sum_comb2_values(&colinears);
        res -= sum_comb2_values(&midpoints);
        res += sum_comb2_values(&midlines);

        res as i32
    }
}
