// Problem 1931
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        // https://leetcode.com/problems/painting-a-grid-with-three-different-colors/?envType=daily-question&envId=2025-05-18
        const MOD: i64 = 10i64.pow(9) + 7;

        let nbits = 1usize << (m * 2);
        let mut dp = vec![vec![0; nbits]; n as usize + 1];

        #[inline]
        fn get_color(bits: usize, row: usize) -> usize {
            (bits >> (row * 2)) & 3
        }

        #[inline]
        fn set_color(bits: usize, row: usize, color: usize) -> usize {
            let shift = row * 2;
            (bits & !(3 << shift)) | ((color & 3) << shift)
        }

        fn recurse(
            m: usize,
            n: usize,
            dp: &mut Vec<Vec<i32>>,
            r: usize,
            c: usize,
            prev_bits: usize,
            curr_bits: usize,
        ) -> i64 {
            // println!("{} {} {} {} {} {}", m, n, i, j, prev_bits, curr_bits);

            if c == n {
                return 1;
            } else if r == 0 && dp[c][prev_bits] != 0 {
                return dp[c][prev_bits] as i64;
            } else if r == m {
                return recurse(m, n, dp, 0, c + 1, curr_bits, 0);
            } else {
                let mut count = 0i64;
                let up_color = if r == 0 {
                    0
                } else {
                    get_color(curr_bits, r - 1)
                };
                let left_color = get_color(prev_bits, r);

                for color in 1..=3 {
                    if color != up_color && color != left_color {
                        count += recurse(
                            m,
                            n,
                            dp,
                            r + 1,
                            c,
                            prev_bits,
                            set_color(curr_bits, r, color),
                        );
                    }
                }

                count %= MOD;

                if r == 0 {
                    dp[c][prev_bits] = count as i32;
                }

                count
            }
        }

        recurse(m as usize, n as usize, &mut dp, 0, 0, 0, 0) as i32
    }
}
