// Problem 808
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        /*
         * https://leetcode.com/problems/soup-servings/solutions/121711/c-java-python-when-n-4800-just-return-1/
         *
         * The key observation is the asymptotic behavior as n goes large
         * so we only need to handle the small number case
         * and result is the expected value
         *
         * A before B -> 1
         * A and B at the same time -> 0.5
         * A after B -> 0
         */

        if n >= 4800 {
            return 1.0;
        }

        // 25ml per serving, and 4800 mL is the max
        // 4800 / 25 = 192 < 200
        let servings = (n + 24) / 25;
        let mut memo = vec![vec![-1.0; 200]; 200];

        fn dfs(memo: &mut Vec<Vec<f64>>, ia: i32, ib: i32) -> f64 {
            if ia <= 0 {
                if ib <= 0 { 0.5 } else { 1.0 }
            } else if ib <= 0 {
                0.0
            } else {
                if memo[ia as usize][ib as usize] < 0.0 {
                    memo[ia as usize][ib as usize] = 0.25
                        * (dfs(memo, ia - 4, ib)
                            + dfs(memo, ia - 3, ib - 1)
                            + dfs(memo, ia - 2, ib - 2)
                            + dfs(memo, ia - 1, ib - 3));
                }

                memo[ia as usize][ib as usize]
            }
        }

        dfs(&mut memo, servings, servings)
    }
}
