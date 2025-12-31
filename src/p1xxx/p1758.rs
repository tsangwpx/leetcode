// Problem 1758
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        // Changes if string starting with zero
        let mut zero_changes = 0;
        // Changes if string starting with one
        let mut one_changes = 0;

        for (idx, ch) in s.bytes().enumerate() {
            let is_zero = (ch == b'0') as i32;
            if idx & 1 == 0 {
                // even position
                zero_changes += 1 - is_zero;
                one_changes += is_zero;
            } else {
                // odd position
                zero_changes += is_zero;
                one_changes += 1 - is_zero;
            }
        }

        one_changes.min(zero_changes)
    }
}
