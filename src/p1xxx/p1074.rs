impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;

        let m = matrix.len();
        let n = matrix[0].len();
        assert!(m >= 1 && n >= 1);

        let mut prefix_sums = vec![vec![0i32; n + 1]; m];
        for j in 0..m {
            let mut psum = 0i32;

            for i in 0..n {
                psum += matrix[j][i];
                prefix_sums[j][i + 1] = psum;
            }

            println!("prefix {}: {:?}", j, prefix_sums[j]);
        }

        // mapping sums to its count
        let mut counter = HashMap::<i32, i32>::with_capacity(m);

        let mut res = 0;

        // the start column, j0, inclusive
        for i0 in 0..=n {
            // the ending column, j1, inclusive
            for i1 in i0 + 1..=n {
                // Reset the counter
                counter.clear();
                counter.insert(0, 1);

                let mut partial = 0;
                for j in 0..m {
                    partial += prefix_sums[j][i1] - prefix_sums[j][i0];

                    let complement = partial - target;
                    println!(
                        "i=[{},{}] j={}: {} {} / {:?}",
                        i0, i1, j, partial, complement, counter
                    );
                    if let Some(s) = counter.get(&complement) {
                        res += s;
                    }

                    counter.entry(partial).and_modify(|s| *s += 1).or_insert(1);
                }
            }
        }

        res
    }
}

fn main() {
    Solution::num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0);
    Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0);
    println!("Hello World");
}
