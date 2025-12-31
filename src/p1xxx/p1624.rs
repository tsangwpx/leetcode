use std::ffi::FromVecWithNulError;

// Problem 1624
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut start = [usize::MAX; 26];
        let mut max_len = -1i32;

        for (stop, ch) in s.bytes().enumerate() {
            let idx = (ch - b'a') as usize;
            assert!(idx < 26);

            if start[idx] == usize::MAX {
                start[idx] = stop;
            } else {
                max_len = max_len.max((stop - start[idx] - 1) as i32);
            }
        }

        max_len
    }
}
