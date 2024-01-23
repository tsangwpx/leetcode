impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        const M: usize = 51 + 2;
        const N: usize = 51 + 2;

        let m = m as usize + 1;
        let n = n as usize + 1;
        let start_row = start_row as usize + 1;
        let start_column = start_column as usize + 1;

        assert!(2 <= m && m < M);
        assert!(2 <= n && n < N);
        assert!(1 <= start_row && start_row < m);
        assert!(1 <= start_column && start_column < n);

        let mut res = 0i64;
        let mut dp0 = vec![vec![0i32; N]; M];
        let mut dp1 = vec![vec![0i32; N]; M];

        dp1[start_row][start_column] = 1;

        for step in 0..max_move {
            std::mem::swap(&mut dp0, &mut dp1);

            // first row
            for i in 1..n {
                res += i64::from(dp0[1][i]);
            }
            // last row
            for i in 1..n {
                let j = m - 1;
                res += i64::from(dp0[j][i]);
            }

            // left and right
            for j in 1..m {
                let i = n - 1;
                res += i64::from(dp0[j][1]);
                res += i64::from(dp0[j][i]);
            }

            res = res % MOD;

            for j in 1..m as usize {
                for i in 1..n as usize {
                    let mut count = 0i64;
                    count += i64::from(dp0[j][i - 1]);
                    count += i64::from(dp0[j][i + 1]);
                    count += i64::from(dp0[j - 1][i]);
                    count += i64::from(dp0[j + 1][i]);
                    dp1[j][i] = (count % MOD) as i32;
                }
            }
            println!("step {}: {:?}", step, dp1);
        }

        (res % MOD) as i32
    }

    pub fn find_paths2(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        assert!(1 <= m && m <= 50);
        assert!(1 <= n && n <= 50);
        assert!(0 <= start_row && start_row < m);
        assert!(0 <= start_column && start_column < n);

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut res = 0i64;
        let mut dp0 = vec![vec![0i32; n as usize]; m as usize];
        let mut dp1 = vec![vec![0i32; n as usize]; m as usize];

        dp1[start_row as usize][start_column as usize] = 1;

        for step in 0..max_move {
            std::mem::swap(&mut dp0, &mut dp1);

            // first row
            for i in 0..n as usize {
                res += i64::from(dp0[0][i]);
            }
            // last row
            for i in 0..n as usize {
                let j = m as usize - 1;
                res += i64::from(dp0[j][i]);
            }

            // left and right
            for j in 0..m as usize {
                let i = n as usize - 1;
                res += i64::from(dp0[j][0]);
                res += i64::from(dp0[j][i]);
            }

            res = res % MOD;

            for j in 0..m as usize {
                for i in 0..n as usize {
                    let mut count = 0i64;
                    if j > 0 {
                        count += i64::from(dp0[j - 1][i]);
                    }
                    if j + 1 < m as usize {
                        count += i64::from(dp0[j + 1][i]);
                    }
                    if i > 0 {
                        count += i64::from(dp0[j][i - 1]);
                    }
                    if i + 1 < n as usize {
                        count += i64::from(dp0[j][i + 1]);
                    }
                    dp1[j][i] = (count % MOD) as i32;
                }
            }
            println!("step {}: {:?}", step, dp1);
        }

        (res % MOD) as i32
    }
}

struct Solution {}

fn main() {
    Solution::find_paths(2, 2, 2, 0, 0);
    Solution::find_paths(1, 3, 3, 0, 1);
    println!("Hello World");
}