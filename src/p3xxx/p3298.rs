// Problem 3298
impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut count = 0i64;

        // Characters we have interests in.
        let mut targets = [false; 26];

        // Frequency balance of target characters
        let mut balance = [0i32; 26];

        for ch in word2.bytes() {
            let offset = (ch - b'a') as usize;
            balance[offset] -= 1;
            targets[offset] = true;
        }

        // Count which character kinds are negative in balance
        let mut missing = targets.iter().filter(|&&s| s).count();

        // The leftmost position of the sliding window.
        let mut left = 0;

        for (idx, ch) in word1.bytes().enumerate() {
            let offset = (ch - b'a') as usize;

            if !targets[offset] {
                // ignore character not in our interests
                continue;
            }

            // bump the balance
            balance[offset] += 1;
            if balance[offset] == 0 {
                // the window now contains just enough characters of such kind
                // reduce the count of missing kinds
                missing -= 1;
            }

            // save the leftmost position
            let prev = left;

            // remove characters from the left of the sliding window
            while left <= idx && missing == 0 {
                let ch2 = word1.bytes().nth(left).unwrap();
                left += 1;
                let offset2 = (ch2 - b'a') as usize;

                if !targets[offset2] {
                    continue;
                }

                balance[offset2] -= 1;
                if balance[offset2] < 0 {
                    // bump the count of missing kinds
                    // this also break the loop
                    missing += 1;
                }
            }

            // now, the possible substring can be formed by picking
            // 1. starting indices in [prev, left)
            // 2. ending indices [idx + 1, word1.len()]
            // the answer is increased by COUNT(starting indices) * COUNT(ending indices)

            let more = (word1.len() - idx) as i64 * (left - prev) as i64;
            count += more;
        }

        count
    }
}
