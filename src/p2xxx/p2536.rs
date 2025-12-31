// Problem 2536
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];

        for query in queries.into_iter() {
            let &[r1, c1, r2, c2] = query.as_slice() else {
                panic!();
            };

            let r1 = r1 as usize;
            let c1 = c1 as usize;
            let r2 = r2 as usize;
            let c2 = c2 as usize;

            res[r1][c1] += 1;

            let h = r2 + 1 < n;
            let v = c2 + 1 < n;

            if h {
                res[r2 + 1][c1] -= 1;
            }

            if v {
                res[r1][c2 + 1] -= 1;
            }

            if h && v {
                res[r2 + 1][c2 + 1] += 1;
            }
        }

        for r in 0..n {
            for c in 0..n.saturating_sub(1) {
                res[r][c + 1] += res[r][c];
            }
        }

        for c in 0..n {
            for r in 0..n.saturating_sub(1) {
                res[r + 1][c] += res[r][c];
            }
        }

        res
    }
}
