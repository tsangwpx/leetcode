impl Solution {
    // 583
    pub fn min_distance(word1: String, word2: String) -> i32 {
        use std::cmp::{max, min};
        use std::convert::TryFrom;

        // Make a the smaller one
        let (a, b) = if word1.len() <= word2.len() {
            (word1.as_bytes(), word2.as_bytes())
        } else {
            (word2.as_bytes(), word1.as_bytes())
        };

        // only the last rows are needed in DP
        const WORD_LENGTH_MAX: usize = 512;
        let mut seq0 = [0u16; WORD_LENGTH_MAX];
        let mut seq1 = [0u16; WORD_LENGTH_MAX];
        assert!(a.len() <= WORD_LENGTH_MAX && b.len() <= WORD_LENGTH_MAX);

        let table = {
            // Seek table to find the first occurrence of a particular character in a.
            let mut table = [a.len(); 256];
            for (idx, &ch) in a.iter().enumerate() {
                if table[ch as usize] == a.len() {
                    table[ch as usize] = idx;
                }
            }
            table
        };

        for j in 0..b.len() {
            let start = table[b[j] as usize];
            if start == a.len() {
                // That character does not exist in a
                continue;
            }

            let mut item = seq0[start];

            for i in start..a.len() {
                // in each step, use the larger one of the following
                // 1. left cell
                // 2. upper cell
                // 3. bumped upper-left cell
                // boundary condition is all zero
                item = max(item, seq0[i]);
                if a[i] == b[j] {
                    item = max(item, if i == 0 { 1 } else { seq0[i - 1] + 1 });
                }
                seq1[i] = item;
            }

            // println!("{:?}", &seq1[..a.len()]);

            // std::mem::swap(&mut seq0, &mut seq1);
            seq0.clone_from_slice(&seq1);
        }

        let total_size = i32::try_from(a.len() + b.len()).unwrap();

        // The maximum length of common subsequence must be the last element
        // Total length subtract the double of common subsequence
        total_size - i32::from(seq0[a.len() - 1]) * 2
    }
}
