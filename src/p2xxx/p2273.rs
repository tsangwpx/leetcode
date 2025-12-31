// Problem 2273
impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        fn letter_frequency(word: &str) -> [i32; 26] {
            let mut dist = [0i32; 26];

            for ch in word.bytes() {
                let idx = (ch - b'a') as usize;
                if idx < dist.len() {
                    dist[idx] += 1;
                }
            }

            dist
        }

        let mut res = vec![];
        let mut prev = None;

        for word in words.into_iter() {
            let freq = letter_frequency(&word);

            let same = if let Some(prev) = prev {
                prev == freq
            } else {
                false
            };

            if same {
                continue;
            }

            prev = Some(freq);
            res.push(word);
        }

        res
    }
}
