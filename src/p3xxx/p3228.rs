// Problem 3228
impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut adj_ones = 0;
        let mut one_count = 0;
        let mut res = 0;
        let mut prev = b'?';

        for item in s.bytes() {
            if item == b'1' {
                adj_ones += 1;
            } else if item == b'0' && prev == b'1' {
                one_count += adj_ones;
                adj_ones = 0;
                res += one_count;
            }

            prev = item;
        }

        res
    }
}
