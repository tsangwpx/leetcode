// Problem 30
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();

        assert!(s.len() >= 1);
        assert!(words.len() >= 1);
        assert!(word_len >= 1);

        use std::collections::HashMap;
        use std::collections::VecDeque;

        if word_len * words.len() > s.len() {
            return vec![];
        }

        // Map word to its count
        let mut word_table = HashMap::new();

        // Count how many occurrence of a word
        let mut word_counter = Vec::with_capacity(words.len());

        for word in words.iter() {
            let word_id = *word_table
                .entry(word.as_str())
                .or_insert(word_counter.len());

            if word_id >= word_counter.len() {
                word_counter.push(1);
            } else {
                word_counter[word_id] += 1;
            }
        }

        let mut res = vec![];
        let mut visited = vec![false; s.len()];
        let mut offsets = VecDeque::with_capacity(words.len());

        for start in 0..s.len() {
            if visited[start] {
                continue;
            }
            visited[start] = true;

            let mut word_count = 0;
            let mut pos = start;

            while let Some(&word_idx) = s.get(pos..pos + word_len).and_then(|s| word_table.get(s)) {
                // Okay. We found a possible word, and this position as done.
                visited[pos] = true;

                if word_counter[word_idx] == 0 {
                    // oops! too many such word.
                    // Popping the offsets deque until we have space for this word
                    while let Some((_, word_idx2)) = offsets.pop_front() {
                        word_counter[word_idx2] += 1;
                        word_count -= 1;
                        if word_idx == word_idx2 {
                            break;
                        }
                    }
                }

                word_counter[word_idx] -= 1;
                word_count += 1;
                offsets.push_back((pos, word_idx));

                if word_count == words.len() {
                    // Yeah! We found a permutation! Find its start position from deque
                    let (start, _) = offsets.front().unwrap();
                    res.push(*start as i32);
                }
                pos += word_len;
            }

            // recover our word_counter, and empty offsets
            while let Some((_, word_idx2)) = offsets.pop_front() {
                word_counter[word_idx2] += 1;
            }
        }

        res
    }
}
