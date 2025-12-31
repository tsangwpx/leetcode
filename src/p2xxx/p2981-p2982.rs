// Problem 2981 & 2982
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        /*!
         * 1. Record the three longest lengths for each alphabet.
         * 2. Aggregate the results by taking the minimum of each alphabet.
         */
        assert!(s.len() >= 3);

        let mut lengths = [[0i32; 3]; 26];

        let mut last = 0u8;
        let mut count = 0;

        for ch in s.bytes() {
            if ch == last {
                count += 1;
            } else {
                count = 1;
                last = ch;
            }

            let idx = (ch - b'a') as usize;
            let ranks = &mut lengths[idx];

            if count > ranks[0] {
                ranks[2] = ranks[1];
                ranks[1] = ranks[0];
                ranks[0] = count;
            } else if count > ranks[1] {
                ranks[2] = ranks[1];
                ranks[1] = count;
            } else if count > ranks[2] {
                ranks[2] = count;
            }
        }

        lengths
            .iter()
            .filter_map(|counts| {
                counts
                    .iter()
                    .copied()
                    .min()
                    .and_then(|count| if count >= 1 { Some(count) } else { None })
            })
            .max()
            .unwrap_or(-1)
    }
}
