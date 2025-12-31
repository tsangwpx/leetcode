// Problem 2938
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut steps = 0;
        let mut left = 0usize;

        for (idx, ch) in s.bytes().enumerate() {
            if ch == b'0' {
                steps += (idx - left) as i64;
                left += 1;
            }
        }

        steps
    }
}
