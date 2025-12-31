// Problem 3363
impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut dp = vec![vec![-1; n + 1]; n + 1];

        #[inline]
        fn collect2(fruits: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            // second child, from (0, n - 1), from top-right cell
            let n = fruits.len();

            if i >= n || j >= n {
                // out of boundary
                return 0;
            }

            if i >= j || i + j < n - 1 {
                // left part of the top-right triangular
                return 0;
            }

            if i == n - 1 && j == n - 1 {
                // dest
                return 0;
            }

            if dp[i][j] < 0 {
                dp[i][j] = fruits[i][j]
                    + collect2(fruits, dp, i + 1, j)
                        .max(collect2(fruits, dp, i + 1, j.wrapping_sub(1)))
                        .max(collect2(fruits, dp, i + 1, j + 1));
                // println!("c2 {} {} {}", dp[i][j], i, j);
            }

            dp[i][j]
        }

        #[inline]
        fn collect3(fruits: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            // third child, from (n - 1, 0), from bottom-left cell

            let n = fruits.len();
            if i >= n || j >= n {
                // out of boundary
                return 0;
            }

            if j >= i || i + j < n - 1 {
                return 0;
            }

            if i == n - 1 && j == n - 1 {
                // dest
                return 0;
            }

            if dp[i][j] < 0 {
                dp[i][j] = fruits[i][j]
                    + collect3(fruits, dp, i, j + 1)
                        .max(collect3(fruits, dp, i.wrapping_sub(1), j + 1))
                        .max(collect3(fruits, dp, i + 1, j + 1));
                // println!("c3 {} {} {}", dp[i][j], i, j);
            }

            dp[i][j]
        }

        let f1 = (0..n).map(|s| fruits[s][s]).sum::<i32>();
        let f2 = collect2(&fruits, &mut dp, 0, n - 1);
        let f3 = collect3(&fruits, &mut dp, n - 1, 0);
        // println!("{} {} {}", f1, f2, f3);

        f1 + f2 + f3
    }
}
