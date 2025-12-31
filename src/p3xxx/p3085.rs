// Problem 3085
impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut counter = word.bytes().fold([0; 26], |mut counter, ch| {
            let idx = (ch - b'a') as usize;
            counter[idx] += 1;
            counter
        });

        counter.sort_unstable();

        let start = {
            let mut idx = 0;
            while idx < counter.len() && counter[idx] == 0 {
                idx += 1;
            }
            idx
        };

        let counter = &counter[start..];

        let mut min = i32::MAX;

        for lower in counter.iter().copied() {
            let mut deleted = 0;
            for freq in counter.iter().copied() {
                if freq < lower {
                    deleted += freq;
                } else {
                    deleted += 0.max(freq - (lower + k));
                }
            }

            min = min.min(deleted);
        }

        min
    }
}
