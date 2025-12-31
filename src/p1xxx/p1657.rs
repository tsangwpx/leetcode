// Problem 1657
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut counter1 = [0i32; 26];
        let mut counter2 = counter1.clone();

        for ch in word1.bytes() {
            counter1[ch as usize - b'a' as usize] += 1;
        }

        for ch in word2.bytes() {
            counter2[ch as usize - b'a' as usize] += 1;
        }

        // If both are consisted of same set of characters,
        // we may apply operation 2 to transform character count to a proper number.
        for i in 0..counter1.len() {
            if (counter1[i] == 0) != (counter2[i] == 0) {
                return false;
            }
        }

        counter1.sort_unstable();
        counter2.sort_unstable();

        counter1 == counter2
    }

    pub fn close_strings2(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        use std::collections::HashMap;

        let counter1 = word1
            .bytes()
            .fold(HashMap::<u8, i32>::new(), |mut counter, ch| {
                counter
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                counter
            });

        let counter2 = word2
            .bytes()
            .fold(HashMap::<u8, i32>::new(), |mut counter, ch| {
                counter
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                counter
            });

        if counter1.len() == counter2.len() && counter1.keys().all(|k| counter2.contains_key(k)) {
            let mut counts1 = counter1.values().copied().collect::<Vec<_>>();
            counts1.sort_unstable();

            let mut counts2 = counter2.values().copied().collect::<Vec<_>>();
            counts2.sort_unstable();

            // If both are consisted of same set of characters,
            // we may apply operation 2 to transform character count to a proper number.

            return counts1 == counts2;
        }

        false
    }
}
