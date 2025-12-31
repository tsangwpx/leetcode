// Problem 58

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        for ch in s.bytes().rev() {
            if ch == b' ' {
                if count > 0 {
                    break;
                }
            } else {
                count += 1
            }
        }

        count
    }

    pub fn length_of_last_word2(s: String) -> i32 {
        s.rsplit_terminator(|ch| ch == ' ')
            .filter_map(|s| {
                if s.len() > 0 {
                    Some(s.len() as i32)
                } else {
                    None
                }
            })
            .next()
            .unwrap()
    }
}
