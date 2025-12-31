use std::ops::Deref;

// Problem 131
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        assert!(s.len() <= 16);

        let mut dp = [[false; 17]; 17];

        // Fixing the end and find palindrome inside
        for (stop, last) in s.bytes().into_iter().enumerate() {
            for start in 0..stop {
                let first = s.bytes().nth(start).unwrap();

                let palindrome = (first == last)
                    && ((stop - start <= 2)
                        || (*dp
                            .get(start + 1)
                            .and_then(|row| row.get(stop - 1))
                            .unwrap_or(&false)));

                dp[start][stop] = palindrome;
            }

            dp[stop][stop] = true;
        }

        let mut buffer = Vec::with_capacity(s.len());
        let mut res = Vec::with_capacity(s.len());

        fn recurse(
            res: &mut Vec<Vec<String>>,
            s: &String,
            dp: &[[bool; 17]; 17],
            start: usize,
            buffer: &mut Vec<String>,
        ) {
            if start == s.len() {
                res.push(buffer.clone());
            }

            // stop is inclusive
            for stop in start..s.len() {
                if dp[start][stop] {
                    // backtraceking
                    buffer.push(s[start..stop + 1].to_owned());
                    recurse(res, s, dp, stop + 1, buffer);
                    buffer.pop();
                }
            }
        }
        recurse(&mut res, &s, &dp, 0, &mut buffer);

        res
    }
}
