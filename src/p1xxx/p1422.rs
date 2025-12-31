// Problem 1422
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut total_zeros = 0;
        let mut total_ones = 0;
        s.bytes().for_each(|s| match s {
            b'0' => total_zeros += 1,
            b'1' => total_ones += 1,
            _ => {}
        });

        let mut max = 0;
        let mut zeros = 0;
        let mut ones = 0;

        // substrings are non-empty
        // skip the last character
        s[..s.len() - 1].bytes().for_each(|s| {
            match s {
                b'0' => zeros += 1,
                b'1' => ones += 1,
                _ => {}
            }
            max = max.max(zeros + total_ones - ones);
        });

        max
    }
}
