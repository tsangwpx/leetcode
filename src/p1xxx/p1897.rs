use std::ffi::FromVecWithNulError;

// Problem 1897
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counter = [0usize; 26];
        words.iter().for_each(|s| {
            s.bytes().for_each(|ch| {
                let idx = (ch - b'a') as usize;
                assert!(idx < 26);
                counter[idx] += 1;
            });
        });

        counter.iter().all(|&c| c % words.len() == 0)
    }
}
