// Problem 2435
impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut dp0 = vec![vec![0i64; k as usize]; n];
        let mut dp1 = dp0.clone();

        dp0[0][0] = 1;

        for i in 0..m {
            for j in 0..n {
                dp1[j].fill(0);

                let item = grid[i][j];

                for rem in 0..k {
                    let count = dp0[j][rem as usize];
                    let rem2 = (rem + item) % k;
                    dp1[j][rem2 as usize] = (dp1[j][rem2 as usize] + count) % MOD;
                }

                if j > 0 {
                    for rem in 0..k {
                        let count = dp1[j - 1][rem as usize];
                        let rem2 = (rem + item) % k;
                        dp1[j][rem2 as usize] = (dp1[j][rem2 as usize] + count) % MOD;
                    }
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.last().unwrap()[0] as i32
    }
}
