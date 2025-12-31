// Problem 960
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();

        // dp[i] is the maximum subsequence in [0..i] inclusive
        let mut maximum = 1;
        let mut dp = vec![1; m]; // single column by itself is a sorted row

        // j2 is the pinned column comparing the previous columns
        for j2 in 0..m {
            // j is the previous column
            'outer: for j in 0..j2 {
                for i in 0..n {
                    if strs[i].bytes().nth(j).unwrap() > strs[i].bytes().nth(j2).unwrap() {
                        continue 'outer;
                    }
                }

                dp[j2] = dp[j2].max(dp[j] + 1);
            }

            maximum = maximum.max(dp[j2]);
        }

        m as i32 - maximum
    }
}
