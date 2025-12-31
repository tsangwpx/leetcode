// Problem 2131
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let mut counter = HashMap::<[u8; 2], i32>::with_capacity(26 * 26);

        for word in words.into_iter() {
            if word.len() >= 2 {
                let a = word.bytes().nth(0).unwrap();
                let b = word.bytes().nth(1).unwrap();
                *counter.entry([a, b]).or_default() += 1;
            }
        }

        let mut len = 0;
        let mut single = false;

        for (word, &count) in counter.iter() {
            if word[0] == word[1] {
                let (q, r) = (count / 2, count % 2);

                len += 4 * q;
                if r != 0 {
                    single = true;
                }
            } else {
                let counterpart = [word[1], word[0]];
                len += counter
                    .get(&counterpart)
                    .copied()
                    .map(|s| s.min(count) * 2)
                    .unwrap_or(0);
            }
        }

        if single {
            len += 2;
        }

        len
    }
}
