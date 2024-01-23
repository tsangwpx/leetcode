impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        assert!(n >= 0);
        assert!(k >= 0);

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut dp = vec![vec![0i32; k as usize + 1]; n as usize + 1];

        // count of zero inverse pair is always one
        for i in 0..=n as usize {
            dp[i][0] = 1;
        }

        for i in 1..=n as usize {
            // the newly added number is `i`

            for j in 1..=k as usize {
                // we need `j` inverse pairs here

                let mut count = 0i64;

                // at least `j - 1` numbers in the previous iteration
                for s in 0..=j.min(i - 1) {
                    count += i64::from(dp[i - 1][j - s]);
                }

                dp[i][j] = (count % MOD) as i32;
            }

            // println!("dp {}: {:?}", i, dp[i]);
        }


        dp[n as usize][k as usize]
    }
}

struct Solution {}

fn main() {
    Solution::min_sum_square_diff(
        vec![1, 2, 3, 4],
        vec![2, 10, 20, 19],
        0, 0,
    );
    println!("Hello World");
}