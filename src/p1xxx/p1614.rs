// Problem 1614
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;

        for ch in s.bytes() {
            if ch == b'(' {
                depth += 1;
                max_depth = max_depth.max(depth);
            } else if ch == b')' {
                depth -= 1;
            }
        }

        max_depth
    }
}
