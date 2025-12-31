// Problem 3333
impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        /*
         * Multi-nominal distribution?
         * with N = k to word.len()
         *
         * seem a dp problem.
         */
        const MOD: i64 = 10i64.pow(9) + 7;

        let segments = {
            let mut segments = vec![];

            for (idx, ch) in word.bytes().enumerate() {
                if word.bytes().nth(idx - 1) == Some(ch) {
                    *segments.last_mut().unwrap() += 1;
                } else {
                    segments.push(1);
                }
            }

            segments
        };
        let total = segments
            .iter()
            .copied()
            .fold(1, |acc, item| (acc as i64 * item as i64) % MOD);

        let k = k as usize;

        if k <= segments.len() {
            return total as i32;
        }

        // println!("{:?}", segments);
        // println!("total {}", total);

        let mut dp0 = vec![0; k];
        let mut dp1 = dp0.clone();
        dp0[0] = 1;

        for (idx, len) in segments.iter().copied().enumerate() {
            let mut sum = 0;

            for i in 0..dp1.len() {
                if i >= 1 {
                    sum += dp0[i - 1] as i64;
                    sum %= MOD;
                }

                if i > len as usize {
                    sum = sum - dp0[i - (len as usize) - 1] + MOD;
                    sum %= MOD;
                }

                dp1[i] = sum;
            }

            std::mem::swap(&mut dp0, &mut dp1);
            // println!("{:?}", dp0);
        }

        let invalid = dp0
            .iter()
            .copied()
            .fold(0i64, |acc, item| (acc + item) % MOD);
        // println!("invalid {}", invalid);

        ((total - invalid + MOD) % MOD) as i32
    }
}
