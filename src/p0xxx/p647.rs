// Problem 647
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let len = s.len();
        let mut count = 0;

        for mid in 0..len {
            count += 1;

            // odd expand
            let mut left = mid.wrapping_sub(1);
            let mut right = mid + 1;

            while left < len
                && right < len
                && s.bytes().nth(left).unwrap() == s.bytes().nth(right).unwrap()
            {
                left = left.wrapping_sub(1);
                right += 1;
                count += 1;
            }

            // even expand
            let mut left = mid;
            let mut right = mid + 1;
            while left < len
                && right < len
                && s.bytes().nth(left).unwrap() == s.bytes().nth(right).unwrap()
            {
                left = left.wrapping_sub(1);
                right += 1;
                count += 1;
            }
        }

        count
    }
}
