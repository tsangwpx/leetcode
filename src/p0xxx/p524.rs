// Problem 524
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        fn is_subseq(haystack: &str, word: &str) -> bool {
            if word.len() > haystack.len() {
                return false;
            }

            let mut pos = 0;
            let mut idx = 0;

            while pos < haystack.len() && idx < word.len() {
                if haystack.bytes().nth(pos).unwrap() == word.bytes().nth(idx).unwrap() {
                    pos += 1;
                    idx += 1;
                } else {
                    pos += 1;
                }
            }

            idx == word.len()
        }

        let mut result: Option<String> = None;

        for word in dictionary.into_iter() {
            if is_subseq(&s, &word) {
                result = if let Some(prev) = result {
                    match prev.len().cmp(&word.len()) {
                        std::cmp::Ordering::Less => Some(word),
                        std::cmp::Ordering::Equal => Some(prev.min(word)),
                        std::cmp::Ordering::Greater => Some(prev),
                    }
                } else {
                    Some(word)
                };
            }
        }

        result.unwrap_or_else(|| "".to_owned())
    }
}
