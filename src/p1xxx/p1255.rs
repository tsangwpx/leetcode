// Problem 1255
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        type CharFreq = [u8; 26];

        let mut words = words;

        words.sort_unstable_by_key(|s| -(s.len() as i32));

        let word_data = words
            .into_iter()
            .map(|s| {
                let mut freq = CharFreq::default();
                let mut word_score = 0;
                for ch in s.bytes() {
                    let idx = (ch - b'a') as usize;
                    freq[idx] += 1;
                    word_score += score[idx];
                }

                (freq, word_score)
            })
            .collect::<Vec<_>>();

        drop(score);

        let letter_freq = {
            let mut freq = CharFreq::default();
            for ch in letters.into_iter() {
                let idx = (ch as u8 - b'a') as usize;
                freq[idx] += 1;
            }
            freq
        };

        let mut dp0 = HashMap::new();
        let mut dp1 = dp0.clone();
        let mut stale = true;
        dp0.insert(letter_freq, 0);

        for (word_freq, word_score) in word_data.into_iter() {
            if stale {
                dp1.clone_from(&dp0);
                stale = false;
            }

            'skip: for (available, score) in dp1.iter() {
                let mut remaining = CharFreq::default();
                for i in 0..26 {
                    if word_freq[i] > available[i] {
                        continue 'skip;
                    }
                    remaining[i] = available[i] - word_freq[i];
                }
                dp0.entry(remaining)
                    .and_modify(|record| *record = (*record).max(word_score + score))
                    .or_insert(word_score + score);
                stale = true;
            }
        }

        dp0.values().copied().max().unwrap()
    }
}
