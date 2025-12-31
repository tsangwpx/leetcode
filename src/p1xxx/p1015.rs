// Problem 1015
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        // not sure what happened
        // the logic is
        // 1. keep suffixing ones to R until R >= K.
        // 2. R = R % K
        // 3. If R = 0, return len.
        // 4. Otherwise, remember this R. Return -1 if this value is encountered again.
        // 5. Go to step 1.

        let mut seen = vec![false; k as usize];

        let mut rem = 0;
        let mut len = 0;

        loop {
            while rem < k {
                rem *= 10;
                rem += 1;
                len += 1;
            }

            rem %= k;
            if rem == 0 {
                return len;
            }

            if seen[rem as usize] {
                return -1;
            }

            seen[rem as usize] = true;
        }
    }
}
