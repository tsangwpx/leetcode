// Problem 3325
impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut counter = [0; 26];

        // the last index at which the character with at least k occurrences
        let mut last_index = None;

        let mut left = 0;
        let mut count = 0;

        for (idx, ch) in s.bytes().enumerate() {
            // update the counter
            let offset = (ch - b'a') as usize;
            counter[offset] += 1;

            if counter[offset] >= k {
                last_index = Some(idx);

                while counter[offset] >= k {
                    let ch2 = s.bytes().nth(left).unwrap();

                    if ch2 == ch && counter[offset] == k {
                        // cant shrink the window anymore
                        break;
                    }

                    let offset2 = (ch2 - b'a') as usize;
                    counter[offset2] -= 1;
                    left += 1;
                }
            }

            if last_index.is_some() {
                // now, the character at left and last_index must be the same
                // and the character in s[left..last_index + 1] has exactly k occurrences.
                // we form substring by picking the starting index between 0 and left (inclusive)
                // and the ending index = idx
                count += left as i32 + 1;
            }
        }

        count
    }
}
